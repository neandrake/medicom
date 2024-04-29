use std::{
    collections::{HashMap, HashSet},
    io::{Read, Write},
};

use bson::{doc, Document};
use dcmpipe_lib::{
    core::{
        charset::{CSRef, DEFAULT_CHARACTER_SET},
        dcmelement::DicomElement,
        dcmobject::DicomRoot,
        defn::{
            dcmdict::DicomDictionary,
            tag::{Tag, TagRef},
            ts::TSRef,
            vr::UN,
        },
        RawValue,
    },
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        tags::{
            AccessionNumber, AdditionalPatientHistory, AdmittingDiagnosesDescription,
            AffectedSOPClassUID, EthnicGroup, IssuerofPatientID, MessageID, ModalitiesinStudy,
            NameofPhysiciansReadingStudy, NumberofPatientRelatedInstances,
            NumberofPatientRelatedSeries, NumberofPatientRelatedStudies,
            NumberofStudyRelatedInstances, NumberofStudyRelatedSeries, Occupation, OtherPatientIDs,
            OtherPatientNames, OtherStudyNumbers, PatientComments, PatientID, PatientsAge,
            PatientsBirthDate, PatientsBirthTime, PatientsName, PatientsSex, PatientsSize,
            PatientsWeight, ProcedureCodeSequence, QueryRetrieveLevel, ReferencedPatientSequence,
            ReferencedStudySequence, ReferringPhysiciansName, SOPClassesinStudy, SOPInstanceUID,
            StudyDate, StudyDescription, StudyID, StudyInstanceUID, StudyTime,
        },
    },
    dimse::{
        assoc::Association,
        commands::messages::CommandMessage,
        error::{AssocError, DimseError},
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

impl<R: Read, W: Write> AssociationDevice<R, W> {
    pub(crate) fn handle_c_find_req(
        &mut self,
        cmd: &CommandMessage,
        dcm: &DicomRoot,
    ) -> Result<(), AssocError> {
        let ctx_id = cmd.ctx_id();
        let msg_id = cmd.get_ushort(&MessageID).map_err(AssocError::ab_failure)?;
        let aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;

        let results = &self.query_c_find_results(dcm)?;

        for result in results {
            let res_rsp = Association::create_cfind_result(ctx_id, msg_id, &aff_sop_class, result)?;
            self.assoc.write_pdu(&res_rsp.0, &mut self.writer)?;
            self.assoc.write_pdu(&res_rsp.1, &mut self.writer)?;
        }

        let end_rsp = Association::create_cfind_end(ctx_id, msg_id, &aff_sop_class)?;
        self.assoc.write_pdu(&end_rsp, &mut self.writer)?;

        Ok(())
    }

    fn query_c_find_results(&self, query: &DicomRoot) -> Result<Vec<DicomRoot>, AssocError> {
        let Some(db) = &self.db else {
            return Ok(Self::create_dummy_results(query, query.ts()));
        };
        let coll = IndexApp::get_dicom_coll(db)
            .map_err(|e| AssocError::ab_failure(DimseError::OtherError(e.into())))?;
        let (query_level, mongo_query, include_keys, meta_keys) =
            Self::dcm_query_to_mongo_query(query)?;

        let query_results = IndexApp::query_docs(&coll, Some(mongo_query))
            .map_err(|e| AssocError::ab_failure(DimseError::OtherError(e.into())))?;

        let group_map = Self::group_results(&query_level, query_results);

        let dcm_results = Self::create_results(query, &include_keys, &meta_keys, &group_map)?;

        Ok(dcm_results)
    }

    fn dcm_query_to_mongo_query(
        dcm: &DicomRoot,
    ) -> Result<(String, Document, Vec<u32>, Vec<u32>), AssocError> {
        let mut query = Document::new();
        let mut include_keys: Vec<u32> = Vec::new();
        let mut meta_keys: Vec<u32> = Vec::new();
        for elem in dcm.flatten() {
            if elem.tag() == QueryRetrieveLevel.tag() {
                continue;
            }
            let Some(tag) = STANDARD_DICOM_DICTIONARY.get_tag_by_number(elem.tag()) else {
                continue;
            };
            if PATIENT_LEVEL_META_TAGS.contains(&tag) || STUDY_LEVEL_META_TAGS.contains(&tag) {
                meta_keys.push(tag.tag());
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

        let query_level = dcm
            .get_value_by_tag(&QueryRetrieveLevel)
            .and_then(|v| v.string().cloned())
            .unwrap_or_else(|| "STUDY".to_owned());

        if query_level == "PATIENT" {
            for tag in PATIENT_LEVEL_TAGS {
                if !include_keys.contains(&tag.tag()) {
                    include_keys.push(tag.tag());
                }
            }
        } else if query_level == "STUDY" {
            for tag in STUDY_LEVEL_TAGS {
                if !include_keys.contains(&tag.tag()) {
                    include_keys.push(tag.tag());
                }
            }
        }

        Ok((query_level, query, include_keys, meta_keys))
    }

    fn group_results(
        query_level: &str,
        query_results: impl Iterator<Item = DicomDoc>,
    ) -> HashMap<String, Vec<DicomDoc>> {
        // The results from mongo are series-level. Group the series results based on the query
        // level specified.
        let mut group_map: HashMap<String, Vec<DicomDoc>> = HashMap::new();
        for result in query_results {
            if query_level == "PATIENT" {
                if let Ok(key) = result.doc().get_str(PATIENT_ID_KEY) {
                    group_map.entry(key.to_owned()).or_default().push(result);
                }
            } else if query_level == "STUDY" {
                if let Ok(key) = result.doc().get_str(STUDY_UID_KEY) {
                    group_map.entry(key.to_owned()).or_default().push(result);
                }
            } else if query_level == "SERIES" {
                if let Ok(key) = result.doc().get_str(SERIES_UID_KEY) {
                    group_map.entry(key.to_owned()).or_default().push(result);
                }
            } else if query_level == "IMAGE" {
                if let Ok(sops) = result.doc().get_array("metadata.sops") {
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
                    if let Ok(sops) = other.doc().get_array("metadata.sops") {
                        for sop in sops {
                            if let Some(sop) = sop.as_str() {
                                sop_instances.insert(sop.to_owned());
                            }
                        }
                    }
                }
                let number_of_studies = study_uids.len();
                let number_of_sops = sop_instances.len();

                if meta_keys.contains(&NumberofPatientRelatedStudies.tag()) {
                    res_root.add_child_with_val(
                        &NumberofPatientRelatedStudies,
                        RawValue::of_string(format!("{number_of_studies}")),
                    );
                }

                if meta_keys.contains(&NumberofPatientRelatedSeries.tag()) {
                    res_root.add_child_with_val(
                        &NumberofPatientRelatedSeries,
                        RawValue::of_string(format!("{number_of_series}")),
                    );
                }

                if meta_keys.contains(&NumberofPatientRelatedInstances.tag()) {
                    res_root.add_child_with_val(
                        &NumberofPatientRelatedInstances,
                        RawValue::of_string(format!("{number_of_sops}")),
                    );
                }

                if meta_keys.contains(&NumberofStudyRelatedSeries.tag()) {
                    res_root.add_child_with_val(
                        &NumberofStudyRelatedSeries,
                        RawValue::of_string(format!("{number_of_series}")),
                    );
                }

                if meta_keys.contains(&NumberofStudyRelatedInstances.tag()) {
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

    fn create_dummy_results(query: &DicomRoot, ts: TSRef) -> Vec<DicomRoot> {
        let q_pid = query
            .get_value_by_tag(&PatientID)
            .and_then(|v| v.string().cloned())
            .unwrap_or_default();
        let q_name = query
            .get_value_by_tag(&PatientsName)
            .and_then(|v| v.string().cloned())
            .unwrap_or_default();

        let mut results = Vec::<DicomRoot>::new();
        for patient in [
            ("477-0101", "SNOW^JON"),
            ("477-0183", "STARK^ROB"),
            ("212-0309", "MARTELL^OBERYN"),
        ] {
            let pid = patient.0;
            let name = patient.1;

            let pid_match = if q_pid.is_empty() {
                false
            } else {
                pid.starts_with(&q_pid) || pid.ends_with(&q_pid)
            };
            let name_match = if q_name.is_empty() {
                false
            } else {
                name.split('^')
                    .any(|p| p.starts_with(&q_name) || p.ends_with(&q_name))
            };
            if !pid_match && !name_match {
                continue;
            }

            let mut result = DicomRoot::new_empty(ts, DEFAULT_CHARACTER_SET);
            result.add_child_with_val(&PatientID, RawValue::of_string(pid));
            result.add_child_with_val(&PatientsName, RawValue::of_string(name));
            results.push(result);
        }
        results
    }
}
