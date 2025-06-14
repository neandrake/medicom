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

mod common;

#[cfg(feature = "stddicom")]
mod parsing_tests {
    use std::{
        convert::TryFrom,
        io::{Cursor, ErrorKind},
    };

    use medicom::{
        core::{
            dcmelement::DicomElement,
            dcmobject::{DicomObject, DicomRoot},
            defn::{
                constants::lookup::MINIMAL_DICOM_DICTIONARY,
                dcmdict::DicomDictionary,
                tag::{Tag, TagPath},
                vl::ValueLength,
                vr,
            },
            read::{stop::ParseStop, ParseError, ParseResult, Parser, ParserBuilder, ParserState},
            values::{ElementWithVr, RawValue},
        },
        dict::{
            stdlookup::STANDARD_DICOM_DICTIONARY,
            tags::{
                ContourData, ContourImageSequence, ContourSequence, FileMetaInformationGroupLength,
                FrameofReferenceUID, Item, ItemDelimitationItem, PatientComments, PatientsName,
                PatientsWeight, PixelData, ROIContourSequence, RTReferencedSeriesSequence,
                RTReferencedStudySequence, ReferencedFrameofReferenceSequence,
                ReferencedImageSequence, ReferencedSOPClassUID, ReferencedSOPInstanceUID,
                ReferencedStudySequence, SOPClassUID, SequenceDelimitationItem, SeriesInstanceUID,
                SharedFunctionalGroupsSequence, SourceImageSequence, SpecificCharacterSet,
                StructureSetROISequence, StructureSetTime, StudyDescription,
            },
            transfer_syntaxes::{ExplicitVRBigEndian, ImplicitVRLittleEndian},
            uids::{CTImageStorage, EnhancedMRImageStorage, MRImageStorage},
        },
    };

    use crate::common::{
        common_stddicom::{
            fixture, is_standard_dcm_file, parse_all_dcmroot_values, parse_all_dicom_files,
            parse_file,
        },
        mock::MockDicomDataset,
        mockdata::{INVALID_VR_ELEMENT, NULL_ELEMENT, STANDARD_HEADER},
    };

    #[test]
    fn test_good_preamble() {
        let mut parser: Parser<'_, MockDicomDataset> = MockDicomDataset::standard_dicom_preamble();

        // reads the preamble, prefix, and first element (mock has no first element)
        let _ = parser
            .next()
            .expect("Should be able to iterate a valid dicom dataset");

        let is_dcm: bool = is_standard_dcm_file(&parser);
        assert!(is_dcm);
    }

    #[test]
    fn test_nonzero_preamble() {
        let mut parser: Parser<'_, MockDicomDataset> = MockDicomDataset::nonzero_preamble();

        // reads the preamble, prefix, and first element (mock has no first element)
        let _ = parser
            .next()
            .expect("Should be able to iterate a valid dicom dataset with non-standard preamble");

        let is_dcm: bool = is_standard_dcm_file(&parser);
        assert!(!is_dcm);
    }

    #[test]
    fn test_bad_dicom_prefix_parser() {
        let mut parser: Parser<'_, MockDicomDataset> = MockDicomDataset::invalid_dicom_prefix();

        // reads the preamble, prefix, and first element
        let result = parser.next();
        assert!(result.is_some());

        let result = result.unwrap();
        assert!(result.is_err());

        let parse_error: ParseError = result.err().unwrap();
        match parse_error {
            ParseError::DetailedError { source, detail } => match *source {
                ParseError::BadDICOMPrefix([68, 79, 67, 77]) => {}
                _ => panic!("{detail:?}"),
            },
            ParseError::BadDICOMPrefix([68, 79, 67, 77]) => {}
            other => panic!("{other:?}"),
        };
    }

    /// Test that failure to read a single element into a `DicomObject` results in a `None` result
    /// rather than an error.
    #[test]
    fn test_invalid_dicom_prefix_is_none() {
        let mut parser: Parser<'_, MockDicomDataset> = MockDicomDataset::invalid_dicom_prefix();
        let parse_result: Option<DicomRoot> =
            DicomRoot::parse(&mut parser).expect("Failed to parse DICOM");
        assert!(parse_result.is_none());
    }

    #[test]
    #[should_panic(expected = "failed to fill whole buffer")]
    fn test_failure_to_read_preamble() {
        let mut parser: Parser<'_, MockDicomDataset> =
            MockDicomDataset::standard_dicom_preamble_diff_startpos_and_short_dataset();

        // reads the preamble, prefix, and first element
        let _first_elem = parser
            .next()
            .expect("Should have returned Some(Err)")
            // This should fail: Failure to read preamble due to not enough data
            .unwrap();

        // should record zero bytes read since the first attempt to read into buffer should fail to fill
        let start_pos: u64 = parser.bytes_read();
        assert_eq!(0, start_pos);
    }

    #[test]
    fn test_unknown_explicit_vr_parses_as_invalid() {
        let mut parser: Parser<'_, MockDicomDataset> =
            MockDicomDataset::build_mock_parser(&[STANDARD_HEADER, INVALID_VR_ELEMENT]);

        // a test dataset that has a regular file-meta section (defines explicit vr) and the first
        // non-file-meta is the  specific character set, followed by zeros. this tests that parsing
        // explicit element of all zeroes results in an UnknownExplicitVR. all other zeroes in an
        // explicit element are valid (though we don't validate that tag numbers are ordered, an all
        // zero tag is not technically valid but itself should't cause a parse error). for an implicit
        // vr transfer syntax the VR will be selected as UN and should parse
        let first_elem: DicomElement = parser
            .find(|x| !x.is_ok() || x.as_ref().unwrap().tag() > SpecificCharacterSet.num())
            .expect("Should have returned Some(Ok(elem))")
            .expect("Should have returned Ok(elem)");

        assert_eq!(&vr::INVALID_VR, first_elem.vr());
    }

    #[test]
    fn test_trailing_zeroes_does_not_error() {
        let mut parser: Parser<'_, MockDicomDataset> =
            MockDicomDataset::build_mock_parser(&[STANDARD_HEADER, NULL_ELEMENT]);

        let first_non_fme: Option<std::result::Result<DicomElement, ParseError>> =
            parser.find(|x| !x.is_ok() || x.as_ref().unwrap().tag() > SpecificCharacterSet.num());

        assert!(first_non_fme.is_none());
    }

    #[test]
    fn test_parser_state_with_std() -> ParseResult<()> {
        test_parser_state(true)
    }

    #[test]
    fn test_parser_state_without_std() -> ParseResult<()> {
        test_parser_state(false)
    }

    fn test_parser_state(with_std: bool) -> ParseResult<()> {
        let stop = &PixelData;
        let dict: &dyn DicomDictionary = if with_std {
            &STANDARD_DICOM_DICTIONARY
        } else {
            &MINIMAL_DICOM_DICTIONARY
        };
        let mut parser = ParserBuilder::default()
            .stop(ParseStop::before(stop))
            .build(
                fixture("gdcm/gdcmConformanceTests/D_CLUNIE_CT1_IVRLE_BigEndian.dcm")?,
                dict,
            );

        assert_eq!(ParserState::DetectTransferSyntax, parser.parser_state());

        let first_elem: DicomElement = parser.next().expect("First element should be Some")?;

        assert_eq!(FileMetaInformationGroupLength.num(), first_elem.tag());

        assert_eq!(ParserState::ReadFileMeta, parser.parser_state());

        for _ in parser.by_ref() {
            // read through the entire dataset
        }

        assert_eq!(ParserState::ReadElement, parser.parser_state());

        // Ability to read dicom elements after FileMetaInformation
        // means that we interpret the transfer syntax properly, as
        // this test file uses implicit VR (FMI is encoded as explicit)

        // subsequent item should not advance reading elements
        let next_elem: Option<ParseResult<DicomElement>> = parser.next();
        assert!(next_elem.is_none());

        // the iterator state should be just after having parsed the stop tag
        let stopped_at_tag: u32 = parser.tag_last_read();
        assert_eq!(
            Tag::format_tag_to_display(stop),
            Tag::format_tag_to_display(stopped_at_tag),
        );

        Ok(())
    }

    #[test]
    fn test_dicom_object_with_std() -> ParseResult<()> {
        test_dicom_object(true)
    }

    #[test]
    fn test_dicom_object_without_std() -> ParseResult<()> {
        test_dicom_object(false)
    }

    fn test_dicom_object(with_std: bool) -> ParseResult<()> {
        let dict: &dyn DicomDictionary = if with_std {
            &STANDARD_DICOM_DICTIONARY
        } else {
            &MINIMAL_DICOM_DICTIONARY
        };

        let mut parser = ParserBuilder::default()
            .stop(ParseStop::before(&PixelData))
            .build(
                fixture("gdcm/gdcmConformanceTests/D_CLUNIE_CT1_IVRLE_BigEndian.dcm")?,
                dict,
            );

        let dcmroot: DicomRoot =
            DicomRoot::parse(&mut parser)?.expect("Failed to parse DICOM elements");
        let sop_class_uid: &DicomObject = dcmroot
            .get_child_by_tag(SOPClassUID.num())
            .expect("Should have SOP Class UID");

        let element: &DicomElement = sop_class_uid.element();

        assert_eq!(
            CTImageStorage.uid(),
            String::try_from(&ElementWithVr(element, &vr::UI))?,
        );

        Ok(())
    }

    #[test]
    fn test_dicom_object_sequences_with_std() -> ParseResult<()> {
        test_dicom_object_sequences(true)
    }

    #[test]
    fn test_dicom_object_sequences_without_std() -> ParseResult<()> {
        test_dicom_object_sequences(false)
    }

    fn test_dicom_object_sequences(with_std: bool) -> ParseResult<()> {
        let dict: &dyn DicomDictionary = if with_std {
            &STANDARD_DICOM_DICTIONARY
        } else {
            &MINIMAL_DICOM_DICTIONARY
        };

        let mut parser = ParserBuilder::default()
            .stop(ParseStop::before(&PixelData))
            .build(
                fixture("gdcm/gdcmConformanceTests/RTStruct_VRDSAsVRUN.dcm")?,
                dict,
            );

        let dcmroot: DicomRoot =
            DicomRoot::parse(&mut parser)?.expect("Failed to parse DICOM elements");

        // StructureSetTime is the last element before a sequence item
        let ss_time: &DicomElement = dcmroot
            .get_child_by_tag(&StructureSetTime)
            .expect("Should have StructureSetTime")
            .element();
        // pull value into local var so it can be typed properly, otherwise it defaults type to &Vec<u8>
        let ss_time_bytes: &[u8] = ss_time.data().as_ref();
        assert_eq!("092108.000".as_bytes(), ss_time_bytes);

        // walk the depths of the first sequence to make sure the structure is setup as we expect
        {
            /* Output from dcmdump
            (3006,0010) SQ (Sequence with undefined length #=1)     # u/l, 1 ReferencedFrameOfReferenceSequence
              (fffe,e000) na (Item with explicit length #=2)          # 1286, 1 Item
                (0020,0052) UI [1.2.246.352.91.0000217.20050503182534.1.1] #  42, 1 FrameOfReferenceUID
                (3006,0012) SQ (Sequence with undefined length #=1)     # u/l, 1 RTReferencedStudySequence
                  (fffe,e000) na (Item with explicit length #=3)          # 1208, 1 Item
                    (0008,1150) UI =RETIRED_DetachedStudyManagementSOPClass #  24, 1 ReferencedSOPClassUID
                    (0008,1155) UI [1.2.246.352.91.0000217.20050503182534]  #  38, 1 ReferencedSOPInstanceUID
                    (3006,0014) SQ (Sequence with undefined length #=1)     # u/l, 1 RTReferencedSeriesSequence
                      (fffe,e000) na (Item with explicit length #=2)          # 1102, 1 Item
                        (0020,000e) UI [1.2.246.352.91.0000217.20050503182534.1] #  40, 1 SeriesInstanceUID
                        (3006,0016) SQ (Sequence with undefined length #=11)    # u/l, 1 ContourImageSequence
                        [11 items, pairs of sop-class and sop-uid references]
                    */

            let tagpath = TagPath::from(vec![
                ReferencedFrameofReferenceSequence.as_node(),
                RTReferencedStudySequence.as_node(),
                RTReferencedSeriesSequence.as_node(),
                ContourImageSequence.as_item_node(11),
                ReferencedSOPInstanceUID.as_node(),
            ]);

            let last_ref_sop_uid_elem: &DicomElement = dcmroot
                .get_child_by_tagpath(&tagpath)
                .expect("Should get element by tagpath")
                .element();

            let last_ref_sop_uid = String::try_from(&ElementWithVr::of(last_ref_sop_uid_elem))?;

            let retrieved_tagpath: TagPath = last_ref_sop_uid_elem.create_tagpath();
            assert_eq!(tagpath, retrieved_tagpath);

            assert_eq!(
                "1.2.246.352.91.0000217.20050503182534671465",
                last_ref_sop_uid,
            );

            // the first sequence item in this object
            let rfor_sq: &DicomObject = dcmroot
                .get_child_by_tag(&ReferencedFrameofReferenceSequence)
                .expect("Should have ReferencedFrameOfReferenceSequence");

            assert_eq!(1, rfor_sq.item_count());
            let item_obj: &DicomObject = rfor_sq.get_item_by_index(1).expect("Have first item");
            assert_eq!(2, item_obj.child_count());
            let item_foruid: &DicomObject = item_obj
                .get_child_by_tag(&FrameofReferenceUID)
                .expect("Have FORUID");
            let item_foruid_bytes: &[u8] = item_foruid.element().data().as_ref();
            assert_eq!(
                "1.2.246.352.91.0000217.20050503182534.1.1\0".as_bytes(),
                item_foruid_bytes,
            );

            assert_eq!(1, rfor_sq.child_count());
            let child_obj: &DicomObject = rfor_sq
                .iter_child_nodes()
                .next()
                .expect("Have first child")
                .1;
            assert_eq!(SequenceDelimitationItem.num(), child_obj.element().tag(),);

            let rtrss_sq: &DicomObject = item_obj
                .get_child_by_tag(&RTReferencedStudySequence)
                .expect("Have RTReferencedStudySequence");
            assert_eq!(1, rtrss_sq.item_count());
            assert_eq!(1, rtrss_sq.child_count());

            let rtrss_sq_item: &DicomObject =
                rtrss_sq.get_item_by_index(1).expect("Have first item");
            assert_eq!(3, rtrss_sq_item.child_count());
            let ref_sopclass: &DicomElement = rtrss_sq_item
                .get_child_by_tag(&ReferencedSOPClassUID)
                .expect("Have ref sop class")
                .element();
            let ref_sopclass_bytes: &[u8] = ref_sopclass.data().as_ref();
            assert_eq!("1.2.840.10008.3.1.2.3.1\0".as_bytes(), ref_sopclass_bytes);

            let ref_sopuid: &DicomElement = rtrss_sq_item
                .get_child_by_tag(&ReferencedSOPInstanceUID)
                .expect("Have ref sop instance uid")
                .element();
            let ref_sopuid_bytes: &[u8] = ref_sopuid.data().as_ref();
            assert_eq!(
                "1.2.246.352.91.0000217.20050503182534\0".as_bytes(),
                ref_sopuid_bytes,
            );

            let rtref_ser_sq: &DicomObject = rtrss_sq_item
                .get_child_by_tag(&RTReferencedSeriesSequence)
                .expect("Have ref series seq");
            assert_eq!(1, rtref_ser_sq.item_count());
            assert_eq!(1, rtref_ser_sq.child_count());

            let rtref_ser_item: &DicomObject =
                rtref_ser_sq.get_item_by_index(1).expect("Have first item");
            assert_eq!(2, rtref_ser_item.child_count());
            let rtref_ser_uid: &DicomElement = rtref_ser_item
                .get_child_by_tag(&SeriesInstanceUID)
                .expect("Have series uid")
                .element();
            let rtref_ser_uid_bytes: &[u8] = rtref_ser_uid.data().as_ref();
            assert_eq!(
                "1.2.246.352.91.0000217.20050503182534.1\0".as_bytes(),
                rtref_ser_uid_bytes,
            );

            let cont_img_sq: &DicomObject = rtref_ser_item
                .get_child_by_tag(&ContourImageSequence)
                .expect("Have contour image seq");
            assert_eq!(11, cont_img_sq.item_count());
            assert_eq!(1, cont_img_sq.child_count());

            let cont_img_sq_child: &DicomObject = cont_img_sq
                .iter_child_nodes()
                .next()
                .expect("Get only child of contour image seq")
                .1;
            assert_eq!(
                SequenceDelimitationItem.num(),
                cont_img_sq_child.element().tag(),
            );
            assert_eq!(0, cont_img_sq_child.child_count());
            assert_eq!(0, cont_img_sq_child.item_count());

            let last_sop_uid: &DicomElement = cont_img_sq
                .get_item_by_index(11)
                .expect("Get last item")
                .get_child_by_tag(&ReferencedSOPInstanceUID)
                .expect("Get last item's ref sop uid")
                .element();
            let last_sop_uid_bytes: &[u8] = last_sop_uid.data().as_ref();
            assert_eq!(
                "1.2.246.352.91.0000217.20050503182534671465\0".as_bytes(),
                last_sop_uid_bytes,
            );
        }

        // test next tag after the first sequence
        let ssroi_sq: &DicomObject = dcmroot
            .get_child_by_tag(&StructureSetROISequence)
            .expect("Should have StructureSetROISequence");
        assert_eq!(4, ssroi_sq.item_count());

        // ContourData's implicit VR is DS, however the first contour in this dataset is encoded
        // explicitly with UN. Verify that it still parses as UN and not DS.
        let contour_data: &DicomElement = dcmroot
            .get_child_by_tag(&ROIContourSequence)
            .expect("Have roi contour seq")
            .get_item_by_index(1)
            .expect("Have first item")
            .get_child_by_tag(&ContourSequence)
            .expect("Have contour sequence")
            .get_item_by_index(1)
            .expect("Have first item")
            .get_child_by_tag(&ContourData)
            .expect("Have contour data")
            .element();

        assert_eq!(&vr::UN, contour_data.vr());
        assert_eq!(ValueLength::Explicit(107_074), contour_data.vl());
        assert_eq!(107_074, contour_data.data().len());

        Ok(())
    }

    #[test]
    fn test_empty_seq_undefined_length_with_std() -> ParseResult<()> {
        test_empty_seq_undefined_length(true)
    }

    #[test]
    fn test_empty_seq_undefined_length_without_std() -> ParseResult<()> {
        test_empty_seq_undefined_length(false)
    }

    /// In this file the `ReferencedStudySequence` and `ReferencedPatientSequence` tags are both `SQ`
    /// elements defined with `UndefinedLength` and contain no data - the first element they have as
    /// contents are `SequenceDelimitationItem` which ends the sequence.
    fn test_empty_seq_undefined_length(with_std: bool) -> ParseResult<()> {
        let dcmroot: DicomRoot = parse_file(
            "gdcm/gdcmConformanceTests/DX_GE_FALCON_SNOWY-VOI.dcm",
            with_std,
        )?;

        let rss_obj: &DicomObject = dcmroot
            .get_child_by_tag(ReferencedStudySequence.num())
            .expect("Should be able to parse ReferencedStudySequence");
        // does contain a child item which is the delimitation item
        assert_eq!(1, rss_obj.child_count());

        let rss_elem: &DicomElement = rss_obj.element();
        assert_eq!(ValueLength::UndefinedLength, rss_elem.vl());

        let sdi_elem: &DicomElement = rss_obj
            .iter_child_nodes()
            .next()
            .expect("Should be able to get single child item")
            .1
            .element();
        assert_eq!(SequenceDelimitationItem.num(), sdi_elem.tag());

        Ok(())
    }

    #[test]
    fn test_private_tag_un_sq_with_std() -> ParseResult<()> {
        test_private_tag_un_sq(true)
    }

    #[test]
    fn test_private_tag_un_sq_without_std() -> ParseResult<()> {
        test_private_tag_un_sq(false)
    }

    /// Private tags with UN VR and `UndefinedLength` should be parsed as sequences. This file uses tags
    /// which are not known to the dictionaries we're parsing with.
    fn test_private_tag_un_sq(with_std: bool) -> ParseResult<()> {
        let dcmroot: DicomRoot = parse_file(
            "gdcm/gdcmConformanceTests/Enhanced_MR_Image_Storage_Illegal_CP246_corrected.dcm",
            with_std,
        )?;

        let private_un_seq_obj: &DicomObject = dcmroot
            .get_child_by_tag(SharedFunctionalGroupsSequence.num())
            .expect("Fixture should have this this tag")
            .get_item_by_index(1)
            .expect("This sequence should have 1 sequence item")
            .get_child_by_tag(0x2005_140Eu32)
            .expect("This sequence should have private element as child");
        assert_eq!(1, private_un_seq_obj.item_count());
        assert_eq!(1, private_un_seq_obj.child_count());

        let private_un_seq_elem: &DicomElement = private_un_seq_obj.element();
        assert_eq!(&vr::UN, private_un_seq_elem.vr());
        assert_eq!(ValueLength::UndefinedLength, private_un_seq_elem.vl());
        assert!(private_un_seq_elem.is_sq_like());
        assert_eq!(0, private_un_seq_elem.data().len());

        let child_obj: &DicomObject = private_un_seq_obj
            .get_item_by_index(1)
            .expect("Private sequence should have one item");
        // The item has 26 elements, plus item delimiter
        assert_eq!(27, child_obj.child_count());
        assert_eq!(
            ItemDelimitationItem.num(),
            *child_obj.iter_child_nodes().last().unwrap().0,
        );

        let sopuid: &DicomElement = child_obj
            .get_child_by_tag(SOPClassUID.num())
            .expect("Should have SOPClassUID child element")
            .element();
        // The MR Image Storage UID is odd-length which means the value is padded with a null byte.
        // Only if we detect the VR as UI (when using standard dictionary) then the value should
        // match exactly when parsed as a string otherwise we have to check it with the null byte.
        if with_std {
            assert_eq!(
                MRImageStorage.uid(),
                String::try_from(&ElementWithVr::of(sopuid))?
            );
        } else {
            assert_eq!(
                format!("{}\u{0}", MRImageStorage.uid()),
                String::try_from(&ElementWithVr::of(sopuid))?,
            );
            // force parsing as UI should match exactly
            assert_eq!(
                MRImageStorage.uid(),
                String::try_from(&ElementWithVr(sopuid, &vr::UI))?,
            );
        }

        Ok(())
    }

    #[test]
    fn test_seq_switch_to_ivrle_with_std() -> ParseResult<()> {
        test_seq_switch_to_ivrle(true)
    }

    #[test]
    fn test_seq_switch_to_ivrle_without_std() -> ParseResult<()> {
        test_seq_switch_to_ivrle(false)
    }

    /// `SequenceDelimitationItem`, `Item`, and `ItemDelimitationItem` are always encoded as IVRLE
    /// despite what the transfer syntax is. This dataset encodes `SourceImageSequence` with an explicit
    /// VR of UN rather than SQ which requires using a parser on the sequence's data field to decode
    /// its contents.
    fn test_seq_switch_to_ivrle(with_std: bool) -> ParseResult<()> {
        let dcmroot: DicomRoot = parse_file(
            "gdcm/gdcmConformanceTests/D_CLUNIE_CT1_IVRLE_BigEndian.dcm",
            with_std,
        )?;
        assert_eq!(&ExplicitVRBigEndian, dcmroot.ts());

        let sis_obj: &DicomObject = dcmroot
            .get_child_by_tag(SourceImageSequence.num())
            .expect("Should have Source Image Sequence");

        // The SourceImageSequence is explicitly set with VR of UN, even though the standard dictionary
        // will properly indicate it's SQ. As it's explicitly UN it will not be parsed as a sequence.
        assert_eq!(0, sis_obj.item_count());

        // Source Image Sequence should not implicitly be parsed as a sequence as it's explicitly
        // encoded with a VR of UN instead of SQ.
        let sis_elem: &DicomElement = sis_obj.element();
        assert_eq!(dcmroot.ts(), sis_elem.ts());

        // Manually parse the contents of Source Image Sequence as a sequence.
        let data: &Vec<u8> = sis_obj.element().data();
        // Initialize the parser to start with Element rather than file-stuff, specifying IVRLE since
        // the contents _must_ be encoded that way in a sequence.
        let mut parser = ParserBuilder::default()
            .state(ParserState::ReadElement)
            .dataset_ts(&ImplicitVRLittleEndian)
            .build(Cursor::new(data), &MINIMAL_DICOM_DICTIONARY);

        let sq_contents_root = DicomRoot::parse(&mut parser)
            .expect("Parse of sequence contents should not error")
            .expect("Parse of sequence contents should result in parsed dicom root");

        let item_obj: &DicomObject = sq_contents_root
            .get_item_by_index(1)
            .expect("Should be able to get child object");

        assert_eq!(2, item_obj.child_count());

        let item_elem: &DicomElement = item_obj.element();
        assert_eq!(Item.num(), item_elem.tag());
        assert_eq!(&ImplicitVRLittleEndian, item_elem.ts());

        for (_tag, inner_obj) in item_obj.iter_child_nodes() {
            let elem: &DicomElement = inner_obj.element();
            // This assertion seems wrong (should be EVRBE) based on Part 5, Section 7.5 --
            // However, the Data Set within the Value Field of the Data Element Item (FFFE,E000) shall
            // be encoded according to the rules conveyed by the Transfer Syntax.
            assert_eq!(&ImplicitVRLittleEndian, elem.ts());
        }

        Ok(())
    }

    #[test]
    fn test_missing_preamble_with_std() -> ParseResult<()> {
        test_missing_preamble(true)
    }

    #[test]
    fn test_missing_preamble_without_std() -> ParseResult<()> {
        test_missing_preamble(false)
    }

    /// This file has no preamble or file meta - should parse as the DICOM default IVRLE
    fn test_missing_preamble(with_std: bool) -> ParseResult<()> {
        let dict: &dyn DicomDictionary = if with_std {
            &STANDARD_DICOM_DICTIONARY
        } else {
            &MINIMAL_DICOM_DICTIONARY
        };

        let mut parser = ParserBuilder::default().build(
            fixture("gdcm/gdcmConformanceTests/OT-PAL-8-face.dcm")?,
            dict,
        );

        let first_elem: DicomElement = parser.next().expect("First element should be parsable")?;

        // first tag is a group length tag
        assert_eq!(0x0008_0000, first_elem.tag());
        // should immediately jump past preamble/prefix, group length, and file meta
        assert_eq!(ParserState::ReadElement, parser.parser_state());
        assert_eq!(&ImplicitVRLittleEndian, parser.ts());

        assert!(parser.file_preamble().is_none());
        assert!(parser.dicom_prefix().is_none());

        // parse the rest of the dataset into an object
        let dcmroot: DicomRoot =
            DicomRoot::parse(&mut parser)?.expect("Failed to parse DICOM elements");
        assert_eq!(32, dcmroot.get_child_count());
        Ok(())
    }

    #[test]
    fn test_undefined_charset_with_std() -> ParseResult<()> {
        test_undefined_charset(true)
    }

    #[test]
    fn test_undefined_charset_without_std() -> ParseResult<()> {
        test_undefined_charset(false)
    }

    /// This file has no Specific Character Set defined and tests the behavior of parsing string values.
    fn test_undefined_charset(with_std: bool) -> ParseResult<()> {
        let dcmroot: DicomRoot = parse_file(
            "gdcm/gdcmConformanceTests/UndefinedValueLengthIllegalNonEncapsulatedTS.dcm",
            with_std,
        )?;

        let scs_elem: &DicomElement = dcmroot
            .get_child_by_tag(SpecificCharacterSet.num())
            .expect("Should have Specific Character Set")
            .element();
        assert!(scs_elem.is_empty());

        let pat_name: &DicomElement = dcmroot
            .get_child_by_tag(PatientsName.num())
            .expect("Should have Patient Name")
            .element();

        let pn: String = String::try_from(&ElementWithVr::of(pat_name))?;
        if with_std {
            assert_eq!("6063^Anon17216", pn);
        } else {
            // The patient name won't be trimmed because without knowing it's a PN element the padding
            // isn't considered in parsing.
            assert_eq!("6063^Anon17216      ", pn);
            // Forcing the parse using a specific VR should trim the value though
            let pn: String = String::try_from(&ElementWithVr(pat_name, &vr::PN))?;
            assert_eq!("6063^Anon17216", pn);
        }

        let pat_com: &DicomElement = dcmroot
            .get_child_by_tag(PatientComments.num())
            .expect("Should have Patient Comments")
            .element();

        let pc: String = String::try_from(&ElementWithVr::of(pat_com))?;
        // this value is a bunch of null bytes. with the standard dictionary this will attempt to parse
        // as a string based on the known VR and be stripped of all null bytes.
        let pc_expected: String = if with_std {
            String::new()
        } else {
            String::from_utf8(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).map_err(|e| {
                ParseError::IOError {
                    source: std::io::Error::new(ErrorKind::InvalidData, format!("{e:?}")),
                }
            })?
        };

        assert_eq!(pc_expected, pc);

        Ok(())
    }

    #[test]
    fn test_rle_with_std() -> ParseResult<()> {
        test_rle(true)
    }

    #[test]
    fn test_rle_without_std() -> ParseResult<()> {
        test_rle(false)
    }

    /// This file is RLE encoded. Eventually test the data can be decompressed properly.
    fn test_rle(with_std: bool) -> ParseResult<()> {
        let _dcmroot: DicomRoot = parse_file(
            "gdcm/gdcmConformanceTests/D_CLUNIE_CT1_RLE_FRAGS.dcm",
            with_std,
        )?;

        Ok(())
    }

    #[test]
    #[cfg(feature = "compress")]
    fn test_deflated_evrle_with_std() -> ParseResult<()> {
        test_deflated_evrle(true)
    }

    #[test]
    #[cfg(feature = "compress")]
    fn test_deflated_evrle_without_std() -> ParseResult<()> {
        test_deflated_evrle(false)
    }

    #[cfg(feature = "compress")]
    fn test_deflated_evrle(with_std: bool) -> ParseResult<()> {
        let _dcmroot: DicomRoot = parse_file(
        "gdcm/gdcmConformanceTests/SequenceWithUndefinedLengthNotConvertibleToDefinedLength.dcm",
        with_std,
    )?;

        Ok(())
    }

    #[test]
    fn test_illegal_cp246_with_std() -> ParseResult<()> {
        test_illegal_cp246(true)
    }

    #[test]
    fn test_illegal_cp246_without_std() -> ParseResult<()> {
        test_illegal_cp246(false)
    }

    fn test_illegal_cp246(with_std: bool) -> ParseResult<()> {
        let dcmroot: DicomRoot = parse_file(
            "gdcm/gdcmConformanceTests/Enhanced_MR_Image_Storage_Illegal_CP246_corrected.dcm",
            with_std,
        )?;

        let ref_sop_class_uid_elem: &DicomElement = dcmroot
            .get_child_by_tag(SharedFunctionalGroupsSequence.num())
            .expect("Should have SharedFunctionalGroupsSequence")
            .get_item_by_index(1)
            .expect("Should have item")
            .get_child_by_tag(ReferencedImageSequence.num())
            .expect("Should have ReferencedImageSequence")
            .get_item_by_index(1)
            .expect("Should have item")
            .get_child_by_tag(ReferencedSOPClassUID.num())
            .expect("Should have ReferencedSOPClassUID")
            .element();

        let ref_sop_class_uid = String::try_from(&ElementWithVr::of(ref_sop_class_uid_elem))?;

        assert_eq!(EnhancedMRImageStorage.uid(), ref_sop_class_uid);

        let ref_sop_class_uid = String::try_from(&ElementWithVr::of(
            dcmroot
                .get_child_by_tagpath(&TagPath::from(vec![
                    SharedFunctionalGroupsSequence.as_item_node(1),
                    ReferencedImageSequence.as_item_node(1),
                    ReferencedSOPClassUID.as_node(),
                ]))
                .expect("Should get by tagpath")
                .element(),
        ))?;

        assert_eq!(EnhancedMRImageStorage.uid(), ref_sop_class_uid);

        let elem_tagpath: TagPath = ref_sop_class_uid_elem.create_tagpath();

        let ref_sop_class_uid = String::try_from(&ElementWithVr::of(
            dcmroot
                .get_child_by_tagpath(&elem_tagpath)
                .expect("Should get by element tagpath")
                .element(),
        ))?;

        assert_eq!(EnhancedMRImageStorage.uid(), ref_sop_class_uid);

        Ok(())
    }

    #[test]
    fn test_incomplete_dicom_file_with_std() -> ParseResult<()> {
        test_incomplete_dicom_file(true)
    }

    #[test]
    fn test_incomplete_dicom_file_without_std() -> ParseResult<()> {
        test_incomplete_dicom_file(false)
    }

    /// This dataset defines an item in (5200,9229)[1].(2005,140E)[1].(0700,0300) which specifies a
    /// very large explicit value length (~50mb) which is not actually present in the dicom file.
    fn test_incomplete_dicom_file(with_std: bool) -> ParseResult<()> {
        let result = parse_file(
            "gdcm/gdcmConformanceTests/Enhanced_MR_Image_Storage_Illegal_CP246.dcm",
            with_std,
        );
        assert!(result.is_err());

        let err = result.unwrap_err();
        if let ParseError::DetailedError { source, detail: _ } = err {
            let source = *source;
            if let ParseError::IOError { source } = source {
                assert_eq!(ErrorKind::UnexpectedEof, source.kind());
            } else {
                panic!("Error should be ParseError::DetailedError(IOError)");
            }
        } else {
            panic!("Error should be ParseError::DetailedError");
        }

        Ok(())
    }

    #[test]
    fn test_no_preamble_start_with_0005_with_std() -> ParseResult<()> {
        test_no_preamble_start_with_0005(true)
    }

    #[test]
    fn test_no_preamble_start_with_0005_without_std() -> ParseResult<()> {
        test_no_preamble_start_with_0005(false)
    }

    /// File has no preamble/prefix and also no File Meta Info header, should default to IVRLE
    fn test_no_preamble_start_with_0005(with_std: bool) -> ParseResult<()> {
        let dcmroot: DicomRoot = parse_file(
            "gdcm/gdcmData/US-IRAD-NoPreambleStartWith0005.dcm",
            with_std,
        )?;

        assert_eq!(&ImplicitVRLittleEndian, dcmroot.ts());

        let study_desc_elem: &DicomElement = dcmroot
            .get_child_by_tag(StudyDescription.num())
            .expect("Should have Study Description tag")
            .element();

        let study_desc = String::try_from(&ElementWithVr::of(study_desc_elem))?;

        if with_std {
            assert_eq!("ABDOMEN", study_desc);
        } else {
            // parsing without dictionary doesn't know VR since parsed as implicit, so it won't know to
            // remove padding value.
            assert_eq!("ABDOMEN ", study_desc);
            // force parsing using the actual VR should trim the padding
            let study_desc: String = String::try_from(&ElementWithVr(study_desc_elem, &vr::LO))?;
            assert_eq!("ABDOMEN", study_desc);
        }

        Ok(())
    }

    #[test]
    fn test_no_dicomv3_preamble_with_std() -> ParseResult<()> {
        test_no_dicomv3_preamble(true)
    }

    #[test]
    fn test_no_dicomv3_preamble_without_std() -> ParseResult<()> {
        test_no_dicomv3_preamble(false)
    }

    /// File has no preamble/prefix
    fn test_no_dicomv3_preamble(with_std: bool) -> ParseResult<()> {
        let dcmroot: DicomRoot = parse_file(
            "gdcm/gdcmData/PICKER-16-MONO2-No_DicomV3_Preamble.dcm",
            with_std,
        )?;

        // check we can read the first element just fine
        let fme_length = u32::try_from(&ElementWithVr::of(
            dcmroot
                .get_child_by_tag(FileMetaInformationGroupLength.num())
                .expect("Should have FileMetaInfo GroupLength tag")
                .element(),
        ))?;

        assert_eq!(84, fme_length);

        Ok(())
    }

    #[test]
    fn test_private_ge_ts_with_std() -> ParseResult<()> {
        test_private_ge_ts(true)
    }

    #[test]
    fn test_private_ge_ts_without_std() -> ParseResult<()> {
        test_private_ge_ts(false)
    }

    fn test_private_ge_ts(with_std: bool) -> ParseResult<()> {
        let _dcmroot: DicomRoot = parse_file(
            "gdcm/gdcmData/PrivateGEImplicitVRBigEndianTransferSyntax16Bits.dcm",
            with_std,
        )?;

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_secured_dicomdir_with_std() -> ParseResult<()> {
        test_secured_dicomdir(true)
    }

    #[test]
    #[ignore]
    fn test_secured_dicomdir_without_std() -> ParseResult<()> {
        test_secured_dicomdir(false)
    }

    /// See Part 10, Section 7.4 on Secure DICOM File Format. File is encrypted with RFC3369
    fn test_secured_dicomdir(with_std: bool) -> ParseResult<()> {
        let _dcmroot: DicomRoot =
            parse_file("gdcm/gdcmData/securedicomfileset/DICOMDIR", with_std)?;

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_secured_image_with_std() -> ParseResult<()> {
        test_secured_image(true)
    }

    #[test]
    #[ignore]
    fn test_secured_image_without_std() -> ParseResult<()> {
        test_secured_image(false)
    }

    /// See Part 10, Section 7.4 on Secure DICOM File Format. File is encrypted with RFC3369
    fn test_secured_image(with_std: bool) -> ParseResult<()> {
        let _dcmroot: DicomRoot =
            parse_file("gdcm/gdcmData/securedicomfileset/IMAGES/IMAGE1", with_std)?;

        Ok(())
    }

    #[test]
    fn test_private_ge_dlx_ts_with_std() -> ParseResult<()> {
        test_private_ge_dlx_ts(true)
    }

    #[test]
    fn test_private_ge_dlx_ts_without_std() -> ParseResult<()> {
        test_private_ge_dlx_ts(false)
    }

    fn test_private_ge_dlx_ts(with_std: bool) -> ParseResult<()> {
        let _dcmroot: DicomRoot =
            parse_file("gdcm/gdcmData/GE_DLX-8-MONO2-PrivateSyntax.dcm", with_std)?;

        Ok(())
    }

    #[test]
    fn test_undefined_item_wrong_vl_with_std() -> ParseResult<()> {
        test_undefined_item_wrong_vl(true)
    }

    #[test]
    fn test_undefined_item_wrong_vl_without_std() -> ParseResult<()> {
        test_undefined_item_wrong_vl(false)
    }

    fn test_undefined_item_wrong_vl(with_std: bool) -> ParseResult<()> {
        let _dcmroot: DicomRoot =
            parse_file("gdcm/gdcmData/BugGDCM2_UndefItemWrongVL.dcm", with_std)?;

        Ok(())
    }

    #[test]
    fn test_uncompressed_even_length_tag_with_std() -> ParseResult<()> {
        test_uncompressed_even_length_tag(true)
    }

    #[test]
    fn test_uncompressed_even_length_tag_without_std() -> ParseResult<()> {
        test_uncompressed_even_length_tag(false)
    }

    fn test_uncompressed_even_length_tag(with_std: bool) -> ParseResult<()> {
        let _dcmroot: DicomRoot = parse_file(
            "gdcm/gdcmData/THERALYS-12-MONO2-Uncompressed-Even_Length_Tag.dcm",
            with_std,
        )?;

        Ok(())
    }

    #[test]
    fn test_dicomdir_with_embedded_icons_with_std() -> ParseResult<()> {
        test_dicomdir_with_embedded_icons(true)
    }

    #[test]
    fn test_dicomdir_with_embedded_icons_without_std() -> ParseResult<()> {
        test_dicomdir_with_embedded_icons(false)
    }

    /// High number of items in a sequence
    fn test_dicomdir_with_embedded_icons(with_std: bool) -> ParseResult<()> {
        let _dcmroot: DicomRoot =
            parse_file("gdcm/gdcmData/dicomdir_With_embedded_icons", with_std)?;

        Ok(())
    }

    #[test]
    fn test_radbw_lossless_with_std() -> ParseResult<()> {
        test_radbw_lossless(true)
    }

    #[test]
    fn test_radbw_lossless_without_std() -> ParseResult<()> {
        test_radbw_lossless(false)
    }

    fn test_radbw_lossless(with_std: bool) -> ParseResult<()> {
        let _dcmroot: DicomRoot = parse_file("gdcm/gdcmData/RadBWLossLess.dcm", with_std)?;

        Ok(())
    }

    #[test]
    fn test_derma_color_lossless_with_std() -> ParseResult<()> {
        test_derma_color_lossless(true)
    }

    #[test]
    fn test_derma_color_lossless_without_std() -> ParseResult<()> {
        test_derma_color_lossless(false)
    }

    fn test_derma_color_lossless(with_std: bool) -> ParseResult<()> {
        let _dcmroot: DicomRoot = parse_file("gdcm/gdcmData/DermaColorLossLess.dcm", with_std)?;

        Ok(())
    }

    #[test]
    fn test_libido_16_acr_nema_volume_with_std() -> ParseResult<()> {
        test_libido_16_acr_nema_volume(true)
    }

    #[test]
    fn test_libido_16_acr_nema_volume_without_std() -> ParseResult<()> {
        test_libido_16_acr_nema_volume(false)
    }

    fn test_libido_16_acr_nema_volume(with_std: bool) -> ParseResult<()> {
        let _dcmroot: DicomRoot =
            parse_file("gdcm/gdcmData/LIBIDO-16-ACR_NEMA-Volume.dcm", with_std)?;

        Ok(())
    }

    #[test]
    fn test_jpeg_lossless_zerolength_sq_with_std() -> ParseResult<()> {
        test_jpeg_lossless_zerolength_sq(true)
    }

    #[test]
    fn test_jpeg_lossless_zerolength_sq_without_std() -> ParseResult<()> {
        test_jpeg_lossless_zerolength_sq(false)
    }

    fn test_jpeg_lossless_zerolength_sq(with_std: bool) -> ParseResult<()> {
        let _dcmroot: DicomRoot = parse_file(
            "gdcm/gdcmData/MARCONI_MxTWin-12-MONO2-JpegLossless-ZeroLengthSQ.dcm",
            with_std,
        )?;

        Ok(())
    }

    #[test]
    fn test_illegal_group2_implicit_ts_with_std() -> ParseResult<()> {
        test_illegal_group2_implicit_ts(true)
    }

    #[test]
    fn test_illegal_group2_implicit_ts_without_std() -> ParseResult<()> {
        test_illegal_group2_implicit_ts(false)
    }

    fn test_illegal_group2_implicit_ts(with_std: bool) -> ParseResult<()> {
        let _dcmroot: DicomRoot =
            parse_file("gdcm/gdcmData/IllegalGroup2ImplicitTS.dcm", with_std)?;

        Ok(())
    }

    #[test]
    #[should_panic(expected = "num bytes not multiple of size of u32")]
    fn test_ul_is_2bytes_with_std() {
        test_ul_is_2bytes(true).unwrap();
    }

    #[test]
    #[should_panic(expected = "num bytes not multiple of size of u32")]
    fn test_ul_is_2bytes_without_std() {
        test_ul_is_2bytes(false).unwrap();
    }

    /// Contains tags (0009,1130), (0009,1131), (0009,1140) with explicit VR of UL but value length is
    /// actually only 2 bytes instead of 4.
    fn test_ul_is_2bytes(with_std: bool) -> ParseResult<()> {
        let dcmroot: DicomRoot = parse_file(
            "gdcm/gdcmData/SIEMENS_GBS_III-16-ACR_NEMA_1-ULis2Bytes.dcm",
            with_std,
        )?;

        let element1: &DicomElement = dcmroot
            .get_child_by_tag(0x0009_1130u32)
            .expect("Element should exist")
            .element();
        assert_eq!(&vr::UL, element1.vr());
        assert_eq!(ValueLength::Explicit(2), element1.vl());
        // should be able to parse the value as u16 since it has 2 bytes
        let element1_val = u16::try_from(&ElementWithVr::of(element1))?;
        assert_eq!(0x0800, element1_val);

        let element2: &DicomElement = dcmroot
            .get_child_by_tag(0x0009_1131u32)
            .expect("Element should exist")
            .element();
        assert_eq!(&vr::UL, element2.vr());
        assert_eq!(ValueLength::Explicit(2), element2.vl());
        // should be able to parse the value as u16 since it has 2 bytes
        let element2_val = u16::try_from(&ElementWithVr::of(element2))?;
        assert_eq!(0x0800, element2_val);

        let element3: &DicomElement = dcmroot
            .get_child_by_tag(0x0009_1140u32)
            .expect("Element should exist")
            .element();
        assert_eq!(&vr::UL, element3.vr());
        assert_eq!(ValueLength::Explicit(2), element3.vl());
        // should be able to parse the value as u16 since it has 2 bytes
        let element3_val = u16::try_from(&ElementWithVr::of(element3))?;
        assert_eq!(0x0800, element3_val);

        // check that we can properly parse the element after the ones with incorrect value length
        let element4: &DicomElement = dcmroot
            .get_child_by_tag(0x0009_1141u32)
            .expect("Element should exist")
            .element();
        assert_eq!(&vr::UL, element4.vr());
        assert_eq!(ValueLength::Explicit(4), element4.vl());

        let element4_val = u32::try_from(&ElementWithVr::of(element4))?;
        assert_eq!(0x2_0000, element4_val);

        // this will return an error
        u32::try_from(&ElementWithVr::of(element1))?;

        Ok(())
    }

    #[test]
    fn test_dicomdir_with_std() -> ParseResult<()> {
        test_dicomdir(true)
    }

    #[test]
    fn test_dicomdir_without_std() -> ParseResult<()> {
        test_dicomdir(false)
    }

    fn test_dicomdir(with_std: bool) -> ParseResult<()> {
        let _dcmroot: DicomRoot = parse_file("dclunie/charsettests/DICOMDIR", with_std)?;

        Ok(())
    }

    #[test]
    #[cfg(feature = "compress")]
    fn test_sq_with_undefined_length_converted_to_defined_length_with_std() -> ParseResult<()> {
        test_sq_with_undefined_length_converted_to_defined_length(true)
    }

    #[test]
    #[cfg(feature = "compress")]
    fn test_sq_with_undefined_length_converted_to_defined_length_without_std() -> ParseResult<()> {
        test_sq_with_undefined_length_converted_to_defined_length(false)
    }

    #[cfg(feature = "compress")]
    fn test_sq_with_undefined_length_converted_to_defined_length(
        with_std: bool,
    ) -> ParseResult<()> {
        let _dcmroot: DicomRoot = parse_file(
            "gdcm/gdcmConformanceTests/SequenceWithUndefinedLengthConvertedToDefinedLength.dcm",
            with_std,
        )?;

        Ok(())
    }

    #[test]
    #[cfg(feature = "compress")]
    fn test_sq_with_undefined_length_unconvertable_to_defined_length_with_std() -> ParseResult<()> {
        test_sq_with_undefined_length_unconvertable_to_defined_length(true)
    }

    #[test]
    #[cfg(feature = "compress")]
    fn test_sq_with_undefined_length_unconvertable_to_defined_length_without_std() -> ParseResult<()>
    {
        test_sq_with_undefined_length_unconvertable_to_defined_length(false)
    }

    /// This is a deflated dataset
    #[cfg(feature = "compress")]
    fn test_sq_with_undefined_length_unconvertable_to_defined_length(
        with_std: bool,
    ) -> ParseResult<()> {
        let _dcmroot: DicomRoot = parse_file(
        "gdcm/gdcmConformanceTests/SequenceWithUndefinedLengthNotConvertibleToDefinedLength.dcm",
        with_std,
    )?;

        Ok(())
    }

    #[test]
    fn test_explicit_vr_for_pub_element_implicit_vr_for_shadow_elements_with_std() -> ParseResult<()>
    {
        test_explicit_vr_for_pub_element_implicit_vr_for_shadow_elements(true)
    }

    #[test]
    fn test_explicit_vr_for_pub_element_implicit_vr_for_shadow_elements_without_std(
    ) -> ParseResult<()> {
        test_explicit_vr_for_pub_element_implicit_vr_for_shadow_elements(false)
    }

    /// This dataset seems partially malformed. It looks like after the `SourceImageSequence` the transfer
    /// syntax of elements at the root switch to being `ImplicitVR` rather than the defined `ExplicitVR`.
    /// It's not clear if this is something that we can handle -- dcmtk also is unable to parse this
    /// though it's not clear if for the same reason.
    ///
    /// See: http://compgroups.net/comp.protocols.dicom/mixing-explicit-and-implicit-transfer-sy/2221446
    ///
    /// >Pre-1996 versions of DCMTK (then still called the European CTN software) had some
    /// >code that would "guess" the transfer syntax for every sequence item and would even
    /// >handle implicit VR big endian encoding. However, the heuristics created more problems
    /// >then they solved.
    fn test_explicit_vr_for_pub_element_implicit_vr_for_shadow_elements(
        with_std: bool,
    ) -> ParseResult<()> {
        let dict: &dyn DicomDictionary = if with_std {
            &STANDARD_DICOM_DICTIONARY
        } else {
            &MINIMAL_DICOM_DICTIONARY
        };

        let mut parser = ParserBuilder::default()
            .stop(ParseStop::after(&SourceImageSequence))
            .build(
                fixture(
                    "gdcm/gdcmData/ExplicitVRforPublicElementsImplicitVRforShadowElements.dcm",
                )?,
                dict,
            );

        let dcmroot: DicomRoot = DicomRoot::parse(&mut parser)?.expect("Parse into object");
        let sis_obj: &DicomObject = dcmroot
            .get_child_by_tag(&SourceImageSequence)
            .expect("Parse SourceImageSequence");

        assert_eq!(1, sis_obj.item_count());

        let tagpath: TagPath = TagPath::from(vec![&SourceImageSequence, &ReferencedSOPInstanceUID]);

        let ref_sop_obj: &DicomObject = dcmroot
            .get_child_by_tagpath(&tagpath)
            .expect("Parse SourceImageSequence.ReferencedSOPInstanceUID");

        let ref_sop_uid: RawValue = ref_sop_obj.element().parse_value()?;
        if let RawValue::Uid(uid) = ref_sop_uid {
            assert_eq!(
                "1.3.46.670589.11.0.0.11.4.2.0.5701.5.5248.2003110619383806273",
                uid,
            );
        } else {
            panic!("Parsed UID should be RawValue::Uid");
        }

        Ok(())
    }

    #[test]
    fn test_explicit_implicit_bogus_iop_with_std() -> ParseResult<()> {
        test_explicit_implicit_bogus_iop(true)
    }

    #[test]
    fn test_explicit_implicit_bogus_iop_without_std() -> ParseResult<()> {
        test_explicit_implicit_bogus_iop(false)
    }

    /// This dataset also randomly switches between implicit and explicit transfer syntax. This one
    /// switches to implicit after Modality, and seems to switch back shortly after `SeriesDescription`.
    fn test_explicit_implicit_bogus_iop(with_std: bool) -> ParseResult<()> {
        let _dcmroot: DicomRoot = parse_file(
            "gdcm/gdcmData/DMCPACS_ExplicitImplicit_BogusIOP.dcm",
            with_std,
        )?;

        Ok(())
    }

    #[test]
    fn test_jpeg_lossless3a_with_std() -> ParseResult<()> {
        test_jpeg_lossless3a(true)
    }

    #[test]
    fn test_jpeg_lossless3a_without_std() -> ParseResult<()> {
        test_jpeg_lossless3a(false)
    }

    /// This dataset has a few extra bytes after the second item/frame in `PixelData`, which don't parse
    /// as a valid `DicomElement`. This test verifies the parser can be configured to return a partial
    /// result, which enables parsing objects which end in garbage data.
    fn test_jpeg_lossless3a(with_std: bool) -> ParseResult<()> {
        let dict: &dyn DicomDictionary = if with_std {
            &STANDARD_DICOM_DICTIONARY
        } else {
            &MINIMAL_DICOM_DICTIONARY
        };

        let path: &str = "gdcm/gdcmData/gdcm-JPEG-LossLess3a.dcm";
        // Parse with default configuration, should result in error.
        let mut parser = ParserBuilder::default().build(fixture(path)?, dict);

        let res: ParseResult<Option<DicomRoot>> = DicomRoot::parse(&mut parser);
        assert!(res.is_err());

        // Parse again but allow a partial DicomObject result. This dataset
        let mut parser = ParserBuilder::default()
            .allow_partial_object(true)
            .build(fixture(path)?, dict);

        let dcmroot = DicomRoot::parse(&mut parser)?.expect("Parse partial object");
        // Verify a bunch of dicom was parsed.
        assert_eq!(106, dcmroot.get_child_count());
        parse_all_dcmroot_values(&dcmroot)?;

        Ok(())
    }

    #[test]
    fn test_kodak_compressed_icon_with_std() -> ParseResult<()> {
        test_kodak_compressed_icon(true)
    }

    #[test]
    fn test_kodak_compressed_icon_without_std() -> ParseResult<()> {
        test_kodak_compressed_icon(false)
    }

    /// This dataset has a pixel data inside `IconImageSequence` - the pixel data itself having 2 items.
    /// This pixel data is undefined length and its items are defined so do not have item delimiters,
    /// but does have a sequence delimiter which should pop off the inner item.
    fn test_kodak_compressed_icon(with_std: bool) -> ParseResult<()> {
        let _dcmroot: DicomRoot = parse_file("gdcm/gdcmData/KODAK_CompressedIcon.dcm", with_std)?;

        Ok(())
    }

    #[test]
    fn test_empty_string_parsed_as_number_with_std() -> ParseResult<()> {
        test_empty_string_parsed_as_number(true)
    }

    #[test]
    fn test_empty_string_parsed_as_number_without_std() -> ParseResult<()> {
        test_empty_string_parsed_as_number(false)
    }

    fn test_empty_string_parsed_as_number(with_std: bool) -> ParseResult<()> {
        let dcmroot: DicomRoot = parse_file("gdcm/gdcmData/SignedShortLosslessBug.dcm", with_std)?;

        let patients_weight = dcmroot
            .get_child_by_tag(PatientsWeight.num())
            .expect("PatientWeight should exist");

        let value: RawValue = patients_weight.element().parse_value()?;
        if let RawValue::Doubles(vals) = value {
            assert!(vals.is_empty());
        } else {
            panic!("PatientsWeight should parse as a list of doubles but was {value:?}");
        }

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_parse_all_dicom_files_with_std() -> ParseResult<()> {
        // Known to fail:
        // - fixtures\gdcm\gdcmConformanceTests\Enhanced_MR_Image_Storage_Illegal_CP246.dcm
        //   | SharedFunctionalGroupsSequence[1].(2005,140E)[1].(0700,0300) has too big ValueLength.
        // - fixtures\gdcm\gdcmData\ExplicitVRforPublicElementsImplicitVRforShadowElements.dcm
        //   | jumbled bytes? last read tag is (0C00,0D00).
        // - fixtures\gdcm\gdcmData\gdcm-JPEG-LossLess3a.dcm
        //   | no delimiter between frames in PixelData, instead reading (B00C,0EB6).
        // - fixtures\gdcm\gdcmData\GE_GENESIS-16-MONO2-WrongLengthItem.dcm
        //   | (0810,0000) has too big ValueLength (GenericGroupLength).
        // - fixtures\gdcm\gdcmData\IM-0001-0066.dcm
        //   | jumbled bytes? last read tag is (6D00,6800).
        // - fixtures\gdcm\gdcmData\JPEGInvalidSecondFrag.dcm
        //   | jumbled bytes in PixelData[2].
        // - [!] fixtures\gdcm\gdcmData\MR_Philips_Intera_PrivateSequenceExplicitVR_in_SQ_2001_e05f_item_wrong_lgt_use_NOSHADOWSEQ.dcm
        //   | jumbled bytes? [!] = error is different from dcmdump.
        // - [!] fixtures\gdcm\gdcmData\MR_Philips_Intera_SwitchIndianess_noLgtSQItem_in_trueLgtSeq.dcm
        //   | jumbled bytes? [!] = error is different from dcmdump.
        // - [!] fixtures\gdcm\gdcmData\PHILIPS_Intera-16-MONO2-Uncompress.dcm
        //   | jumbled bytes? [!] = error is different from dcmdump.
        // - [!] fixtures\gdcm\gdcmData\securedicomfileset\DICOMDIR
        //   | DICOM is encrypted. [!] error is different from dcmdump.
        // - [!] fixtures\gdcm\gdcmData\securedicomfileset\IMAGES\IMAGE1
        //   | DICOM is encrypted. [!] error is different from dcmdump.
        // - fixtures\gdcm\gdcmData\SIEMENS_GBS_III-16-ACR_NEMA_1-ULis2Bytes.dcm
        //   | Element parse error: (0009,1130)+ are UL but only have 2 bytes.
        // - fixtures\gdcm\gdcmData\SIEMENS_MAGNETOM-12-MONO2-GDCM12-VRUN.dcm
        //   | (0009,1214) has too big ValueLength.
        // - [!] fixtures\gdcm\gdcmData\SIEMENS_MAGNETOM-12-MONO2-Uncompressed.dcm
        //   | Element parse error: (0009,1113) is UL but only 2 bytes. [!] error is different from
        //   dcmdump. dcmdump fails the rest of the file due to jumbled bytes and eventually a value
        //   length too big.
        // - ReferencedPerformedProcedureStepSequence[1].(0500,0400) has too big ValueLength.
        let num_failed: usize = parse_all_dicom_files(true)?;
        if num_failed > 0 {
            Err(ParseError::IOError {
                source: std::io::Error::new(
                    ErrorKind::InvalidData,
                    format!("Failed to parse DICOM files: {num_failed}"),
                ),
            })
        } else {
            Ok(())
        }
    }

    #[test]
    #[ignore]
    fn test_parse_all_dicom_files_without_std() -> ParseResult<()> {
        let num_failed: usize = parse_all_dicom_files(false)?;
        if num_failed > 0 {
            Err(ParseError::IOError {
                source: std::io::Error::new(
                    ErrorKind::InvalidData,
                    format!("Failed to parse DICOM files: {num_failed}"),
                ),
            })
        } else {
            Ok(())
        }
    }
}
