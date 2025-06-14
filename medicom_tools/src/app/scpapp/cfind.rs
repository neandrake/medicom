/*
   Copyright 2024-2025 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use std::{
    collections::{HashMap, HashSet},
    io::{Read, Write},
    str::FromStr,
};

use bson::{doc, Document};
use medicom::{
    core::{
        charset::CSRef,
        dcmelement::DicomElement,
        dcmobject::DicomRoot,
        defn::{
            dcmdict::DicomDictionary,
            tag::{Tag, TagRef},
            ts::TSRef,
            vr::UN,
        },
        values::RawValue,
    },
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        tags::{
            AccessionNumber, AdditionalPatientHistory, AdmittingDiagnosesDescription, EthnicGroup,
            InstanceCreationDate, InstanceNumber, IssuerofPatientID, ModalitiesinStudy, Modality,
            NameofPhysiciansReadingStudy, NumberofPatientRelatedInstances,
            NumberofPatientRelatedSeries, NumberofPatientRelatedStudies,
            NumberofStudyRelatedInstances, NumberofStudyRelatedSeries, Occupation, OtherPatientIDs,
            OtherPatientNames, OtherStudyNumbers, PatientComments, PatientID, PatientsAge,
            PatientsBirthDate, PatientsBirthTime, PatientsName, PatientsSex, PatientsSize,
            PatientsWeight, ProcedureCodeSequence, QueryRetrieveLevel, ReferencedPatientSequence,
            ReferencedStudySequence, ReferringPhysiciansName, SOPClassUID, SOPClassesinStudy,
            SOPInstanceUID, SeriesDate, SeriesDescription, SeriesInstanceUID, SeriesNumber,
            SeriesTime, StudyDate, StudyDescription, StudyID, StudyInstanceUID, StudyTime,
        },
    },
    dimse::{
        assoc::QueryLevel,
        commands::{messages::CommandMessage, CommandStatus},
        error::{AssocError, DimseError},
        svcops::FindSvcOp,
    },
};

use crate::app::{
    indexapp::{DicomDoc, IndexApp},
    scpapp::AssociationDevice,
};

static PATIENT_ID_KEY: &str = "00100020";
static STUDY_UID_KEY: &str = "0020000D";
static SERIES_UID_KEY: &str = "0020000E";

static PATIENT_LEVEL_TAGS: [TagRef; 11] = [
    &PatientsName,
    &PatientID,
    &IssuerofPatientID,
    &ReferencedPatientSequence,
    &PatientsBirthDate,
    &PatientsBirthTime,
    &PatientsSex,
    &OtherPatientIDs,
    &OtherPatientNames,
    &EthnicGroup,
    &PatientComments,
];
static PATIENT_LEVEL_META_TAGS: [TagRef; 3] = [
    &NumberofPatientRelatedStudies,
    &NumberofPatientRelatedSeries,
    &NumberofPatientRelatedInstances,
];

static STUDY_LEVEL_TAGS: [TagRef; 17] = [
    &StudyDate,
    &StudyTime,
    &AccessionNumber,
    &StudyID,
    &StudyInstanceUID,
    &ReferringPhysiciansName,
    &StudyDescription,
    &ProcedureCodeSequence,
    &NameofPhysiciansReadingStudy,
    &AdmittingDiagnosesDescription,
    &ReferencedStudySequence,
    &PatientsAge,
    &PatientsSize,
    &PatientsWeight,
    &Occupation,
    &AdditionalPatientHistory,
    &OtherStudyNumbers,
];
static STUDY_LEVEL_META_TAGS: [TagRef; 4] = [
    &NumberofStudyRelatedSeries,
    &NumberofStudyRelatedInstances,
    &ModalitiesinStudy,
    &SOPClassesinStudy,
];

static SERIES_LEVEL_TAGS: [TagRef; 6] = [
    &Modality,
    &SeriesInstanceUID,
    &SeriesNumber,
    &SeriesDescription,
    &SeriesDate,
    &SeriesTime,
];

static IMAGE_LEVEL_TAGS: [TagRef; 4] = [
    &SOPClassUID,
    &SOPInstanceUID,
    &InstanceNumber,
    &InstanceCreationDate,
];

pub(crate) struct MongoQuery {
    pub ql: QueryLevel,
    pub query: Document,
    pub include_keys: Vec<u32>,
    pub meta_keys: Vec<u32>,
}

pub(crate) struct QueryResults {
    pub query: MongoQuery,
    pub group_map: HashMap<String, Vec<DicomDoc>>,
}

impl<R: Read, W: Write> AssociationDevice<R, W> {
    pub(crate) fn handle_c_find_req(
        &mut self,
        op: &mut FindSvcOp,
        cmd: &CommandMessage,
    ) -> Result<(), AssocError> {
        let pdu_max_snd_size = self.assoc.common().get_pdu_max_snd_size();
        let dcm_query =
            op.process_req(cmd, self.assoc.common(), &mut self.reader, &mut self.writer)?;

        let query_results = &self.query_database(&dcm_query)?;
        let dcm_results = Self::create_results(
            &dcm_query,
            &query_results.query.include_keys,
            &query_results.query.meta_keys,
            &query_results.group_map,
        )?;

        for result in &dcm_results {
            op.write_response(&mut self.writer, pdu_max_snd_size, result)?;
        }
        op.end_response(
            &mut self.writer,
            pdu_max_snd_size,
            &CommandStatus::success(),
        )?;

        Ok(())
    }

    pub(crate) fn query_database(&self, dcm_query: &DicomRoot) -> Result<QueryResults, AssocError> {
        let Some(db) = &self.db else {
            return Err(AssocError::ab_failure(DimseError::ApplicationError(
                "Failed connecting to database".into(),
            )));
        };

        let coll = IndexApp::get_dicom_coll(db)
            .map_err(|e| AssocError::ab_failure(DimseError::ApplicationError(e.into())))?;
        let mongo_query = Self::convert_dcm_query_to_mongo_query(dcm_query)?;

        let query_results = IndexApp::query_docs(&coll, mongo_query.query.clone())
            .map_err(|e| AssocError::ab_failure(DimseError::ApplicationError(e.into())))?;

        // XXX: Stream results from mongo to avoid pulling all into memory?
        let group_map = Self::group_results(mongo_query.ql, query_results);

        Ok(QueryResults {
            query: mongo_query,
            group_map,
        })
    }

    pub(crate) fn convert_dcm_query_to_mongo_query(
        dcm: &DicomRoot,
    ) -> Result<MongoQuery, AssocError> {
        let mut query = Document::new();
        let mut include_keys: Vec<u32> = Vec::new();
        let mut meta_keys: Vec<u32> = Vec::new();
        for elem in dcm.flatten() {
            if elem.tag() == QueryRetrieveLevel.num() {
                continue;
            }
            let Some(tag) = STANDARD_DICOM_DICTIONARY.get_tag_by_number(elem.tag()) else {
                continue;
            };
            if PATIENT_LEVEL_META_TAGS.contains(&tag) || STUDY_LEVEL_META_TAGS.contains(&tag) {
                meta_keys.push(tag.num());
                continue;
            }

            let elem_key = IndexApp::tag_to_key(elem.tag());
            include_keys.push(elem.tag());
            if !elem.is_empty() {
                let val = elem
                    .parse_value()
                    .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?;
                if let Some(string) = val.string() {
                    if !string.is_empty() {
                        if tag == &SOPInstanceUID {
                            let sop_in = doc! {
                                "$in": string,
                            };
                            query.insert("metadata.sops", sop_in);
                        } else {
                            let string = string.replace('*', ".*").replace(['/', '\\'], "");
                            let regex = doc! {
                                "$regex": string,
                                "$options": "i",
                            };
                            query.insert(elem_key, regex);
                        }
                    }
                }
            }
        }

        let ql = if let Some(ql_val) = dcm.get_value_by_tag(&QueryRetrieveLevel) {
            if let Some(ql_val_str) = ql_val.string() {
                QueryLevel::from_str(ql_val_str.as_str())
                    .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?
            } else {
                QueryLevel::Study
            }
        } else {
            QueryLevel::Study
        };

        if ql.include_patient_tags() {
            for tag in PATIENT_LEVEL_TAGS {
                if !include_keys.contains(&tag.num()) {
                    include_keys.push(tag.num());
                }
            }
        }
        if ql.include_study_tags() {
            for tag in STUDY_LEVEL_TAGS {
                if !include_keys.contains(&tag.num()) {
                    include_keys.push(tag.num());
                }
            }
        }
        if ql.include_series_tags() {
            for tag in SERIES_LEVEL_TAGS {
                if !include_keys.contains(&tag.num()) {
                    include_keys.push(tag.num());
                }
            }
        }
        if ql.include_image_tags() {
            for tag in IMAGE_LEVEL_TAGS {
                if !include_keys.contains(&tag.num()) {
                    include_keys.push(tag.num());
                }
            }
        }

        Ok(MongoQuery {
            ql,
            query,
            include_keys,
            meta_keys,
        })
    }

    pub(crate) fn group_results(
        ql: QueryLevel,
        query_results: impl Iterator<Item = DicomDoc>,
    ) -> HashMap<String, Vec<DicomDoc>> {
        // The results from mongo are series-level. Group the series results based on the query
        // level specified.
        let mut group_map: HashMap<String, Vec<DicomDoc>> = HashMap::new();
        for result in query_results {
            if ql == QueryLevel::Patient {
                if let Ok(key) = result.doc().get_str(PATIENT_ID_KEY) {
                    group_map.entry(key.to_owned()).or_default().push(result);
                }
            } else if ql == QueryLevel::Study {
                if let Ok(key) = result.doc().get_str(STUDY_UID_KEY) {
                    group_map.entry(key.to_owned()).or_default().push(result);
                }
            } else if ql == QueryLevel::Series {
                if let Ok(key) = result.doc().get_str(SERIES_UID_KEY) {
                    group_map.entry(key.to_owned()).or_default().push(result);
                }
            } else if ql == QueryLevel::Image {
                let sops_doc = result
                    .doc()
                    .get_document("metadata")
                    .and_then(|m| m.get_array("sops"));
                if let Ok(sops) = sops_doc {
                    for sop in sops {
                        let Some(sop) = sop.as_str() else {
                            continue;
                        };

                        // XXX: Cloning the series result for each SOP...
                        group_map
                            .entry(sop.to_owned())
                            .or_default()
                            .push(result.clone());
                    }
                }
            }
        }
        group_map
    }

    fn create_results(
        query: &DicomRoot,
        include_keys: &[u32],
        meta_keys: &[u32],
        group_map: &HashMap<String, Vec<DicomDoc>>,
    ) -> Result<Vec<DicomRoot>, AssocError> {
        let mut dcm_results: Vec<DicomRoot> = Vec::new();
        for results in group_map.values() {
            if let Some(result) = results.first() {
                let mut res_root = Self::mongo_doc_to_dcm_root(
                    result.doc(),
                    include_keys,
                    query.ts(),
                    query.cs(),
                )?;

                let number_of_series = results.len();

                let mut study_uids: HashSet<String> = HashSet::new();
                let mut sop_instances: HashSet<String> = HashSet::new();
                for other in results {
                    if let Ok(study_uid) = other.doc().get_str(STUDY_UID_KEY) {
                        study_uids.insert(study_uid.to_owned());
                    }
                    let sops_doc = other
                        .doc()
                        .get_document("metadata")
                        .and_then(|m| m.get_array("sops"));
                    if let Ok(sops) = sops_doc {
                        for sop in sops {
                            if let Some(sop) = sop.as_str() {
                                sop_instances.insert(sop.to_owned());
                            }
                        }
                    }
                }
                let number_of_studies = study_uids.len();
                let number_of_sops = sop_instances.len();

                if meta_keys.contains(&NumberofPatientRelatedStudies.num()) {
                    res_root.add_child_with_val(
                        &NumberofPatientRelatedStudies,
                        RawValue::of_string(format!("{number_of_studies}")),
                    );
                }

                if meta_keys.contains(&NumberofPatientRelatedSeries.num()) {
                    res_root.add_child_with_val(
                        &NumberofPatientRelatedSeries,
                        RawValue::of_string(format!("{number_of_series}")),
                    );
                }

                if meta_keys.contains(&NumberofPatientRelatedInstances.num()) {
                    res_root.add_child_with_val(
                        &NumberofPatientRelatedInstances,
                        RawValue::of_string(format!("{number_of_sops}")),
                    );
                }

                if meta_keys.contains(&NumberofStudyRelatedSeries.num()) {
                    res_root.add_child_with_val(
                        &NumberofStudyRelatedSeries,
                        RawValue::of_string(format!("{number_of_series}")),
                    );
                }

                if meta_keys.contains(&NumberofStudyRelatedInstances.num()) {
                    res_root.add_child_with_val(
                        &NumberofStudyRelatedInstances,
                        RawValue::of_string(format!("{number_of_sops}")),
                    );
                }

                // If the query is looking for a specific SOP Instance UID then make sure that the
                // result shows the SOP that was queried for. This is ~hackish, since the database
                // does not store records for every SOP but instead every series.
                if let Some(query_sop) = query.get_value_by_tag(&SOPInstanceUID) {
                    let query_sop = query_sop.string().cloned().unwrap_or_default();
                    if !query_sop.is_empty() {
                        if let Some(sop_obj) = res_root.get_child_by_tag_mut(&SOPInstanceUID) {
                            sop_obj
                                .element_mut()
                                .encode_val(RawValue::of_string(query_sop))
                                .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?;
                        }
                    }
                }

                if res_root.get_child_count() > 0 {
                    dcm_results.push(res_root);
                }
            }
        }
        Ok(dcm_results)
    }

    fn mongo_doc_to_dcm_root(
        doc: &Document,
        include_keys: &[u32],
        ts: TSRef,
        cs: CSRef,
    ) -> Result<DicomRoot, AssocError> {
        let mut res_root = DicomRoot::new_empty(ts, cs);
        for key in include_keys {
            let tag = *key;
            let key = IndexApp::tag_to_key(tag);

            let vr = STANDARD_DICOM_DICTIONARY
                .get_tag_by_number(tag)
                .and_then(Tag::implicit_vr)
                .unwrap_or(&UN);
            let mut res_elem = DicomElement::new_empty(tag, vr, ts);
            if let Some(value) = doc.get(key) {
                if let Some(string) = value.as_str() {
                    res_elem
                        .encode_val(RawValue::of_string(string))
                        .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?;
                } else if let Some(int) = value.as_i32() {
                    res_elem
                        .encode_val(RawValue::of_int(int))
                        .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?;
                } else if let Some(long) = value.as_i64() {
                    res_elem
                        .encode_val(RawValue::of_long(long))
                        .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?;
                } else if let Some(double) = value.as_f64() {
                    res_elem
                        .encode_val(RawValue::of_double(double))
                        .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?;
                }
            }
            if !res_elem.is_empty() {
                res_root.add_element(res_elem);
            }
        }
        Ok(res_root)
    }
}
