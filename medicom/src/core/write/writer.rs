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

use std::io::Write;

use crate::core::{
    charset::CSRef,
    dcmelement::DicomElement,
    dcmobject::DicomRoot,
    defn::{
        constants::{
            tags::{FILE_META_GROUP_END, FILE_META_INFORMATION_GROUP_LENGTH},
            ts::{ExplicitVRLittleEndian, ImplicitVRBigEndian, ImplicitVRLittleEndian},
        },
        is_parent_priv_sq, is_sq_delim,
        ts::TSRef,
        vl::{ValueLength, UNDEFINED_LENGTH},
        vr::{self, VRRef, OB},
    },
    read::ParseError,
    values::RawValue,
    write::{ds::dataset::Dataset, error::WriteError},
    DICOM_PREFIX, FILE_PREAMBLE_LENGTH,
};

pub type WriteResult<T> = core::result::Result<T, WriteError>;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum WriterState {
    WritePreamble,
    WriteGroupLength,
    WriteFileMeta,
    WriteElement,
}

#[derive(Debug)]
pub struct Writer<W: Write> {
    pub(crate) dataset: Dataset<W>,

    pub(crate) state: WriterState,

    pub(crate) bytes_written: u64,

    pub(crate) ts: TSRef,

    pub(crate) cs: CSRef,

    /// The file preamble to write to the dataset. Not all datasets may have a preamble.
    /// If a file preamble is specified then the `"DICM"` prefix will be written immediately
    /// after the file preamble is written.
    pub(crate) file_preamble: Option<[u8; FILE_PREAMBLE_LENGTH]>,
}

impl<W: Write> Writer<W> {
    /// Get the number of bytes read from the dataset.
    pub fn bytes_written(&self) -> u64 {
        self.bytes_written
    }

    /// Get the current state of the parser.
    pub fn write_state(&self) -> WriterState {
        self.state
    }

    /// Get the transfer syntax the dataset is encoded in.
    pub fn ts(&self) -> TSRef {
        self.ts
    }

    /// Get the character set string values are encoded in.
    pub fn cs(&self) -> CSRef {
        self.cs
    }

    /// Creates a new `DicomElement` with the given value encoded with the given VR.
    ///
    /// # Errors
    /// Encoding the value may fail.
    pub fn create_element<T>(&self, tag: T, vr: VRRef, value: RawValue) -> WriteResult<DicomElement>
    where
        T: Into<u32>,
    {
        let tag: u32 = tag.into();
        let mut e = DicomElement::new_empty(tag, vr, self.ts);
        e.encode_val(value)?;
        Ok(e)
    }

    /// Consumes self and return the wrapped dataset field.
    pub fn into_dataset(self) -> W {
        self.dataset.into_inner()
    }

    /// Flattens the given `DicomRoot` elements into a stream of `DicomElement` and writes the
    /// resulting elements into the dataset.
    ///
    /// # Errors
    /// Errors may occur writing to the dataset.
    pub fn write_dcmroot(&mut self, dcmroot: &DicomRoot) -> WriteResult<usize> {
        let elements = dcmroot.flatten();
        self.write_elements(elements.into_iter())
    }

    /// Write the iterator of `DicomElement` to the dataset. If the `WriteState` is set to any
    /// valid state for file media, this will handle appropriate encoding for file meta group.
    ///
    /// # Errors
    /// Errors may occur writing to the dataset.
    pub fn write_elements<'a, I>(&mut self, elements: I) -> WriteResult<usize>
    where
        I: Iterator<Item = &'a DicomElement>,
    {
        let mut bytes_written: usize = 0;

        if self.state == WriterState::WritePreamble {
            if let Some(preamble) = self.file_preamble {
                self.dataset.write_all(&preamble)?;
                bytes_written += preamble.len();
                self.dataset.write_all(DICOM_PREFIX)?;
                bytes_written += DICOM_PREFIX.len();
            }
            self.state = WriterState::WriteFileMeta;
        }

        let mut fm_elements: Vec<&DicomElement> = Vec::new();
        for element in elements {
            // Collect all the FileMeta elements to write them in one go, as their total byte
            // length is needed for the first element, FileMetaInformationGroupLength.
            if self.state == WriterState::WriteFileMeta {
                if element.tag() <= FILE_META_GROUP_END {
                    // Ignore FileMetaInformationGroupLength in place of one made below.
                    if element.tag() != FILE_META_INFORMATION_GROUP_LENGTH {
                        fm_elements.push(element);
                    }
                    continue;
                }

                bytes_written += self.write_fm_elements(fm_elements.as_slice())?;
                // The list of FileMeta elements are no longer needed.
                fm_elements.clear();

                // Flip state to write standard elements, and fall-through. In the condition for
                // getting to this state the `element` value is non-FileMeta and hasn't been
                // written out yet.
                self.state = WriterState::WriteElement;
            }

            bytes_written += Writer::write_element(&mut self.dataset, self.ts, element)?;
        }

        // If the input elements only consist of FileMeta elements then the above loop will never
        // result in writing any elements as they're being collected into `fm_elements`.
        if self.state == WriterState::WriteFileMeta && !fm_elements.is_empty() {
            bytes_written += self.write_fm_elements(fm_elements.as_slice())?;
        }

        self.bytes_written += u64::try_from(bytes_written).unwrap_or_default();
        Ok(bytes_written)
    }

    /// Write the iterator of `DicomElement` to the dataset. If the `WriteState` is set to any
    /// valid state for file media, this will handle appropriate encoding for file meta group.
    ///
    /// # Notes
    /// This is the same as `write_elements()` but takes an iterator over owned elements.
    ///
    /// This is an unfortunate copy/paste.
    ///
    /// # Errors
    /// Errors may occur writing to the dataset.
    pub fn write_owned_elements<I>(&mut self, elements: I) -> WriteResult<usize>
    where
        I: Iterator<Item = DicomElement>,
    {
        let mut bytes_written: usize = 0;

        if self.state == WriterState::WritePreamble {
            if let Some(preamble) = self.file_preamble {
                self.dataset.write_all(&preamble)?;
                bytes_written += preamble.len();
                self.dataset.write_all(DICOM_PREFIX)?;
                bytes_written += DICOM_PREFIX.len();
            }
            self.state = WriterState::WriteFileMeta;
        }

        let mut fm_elements: Vec<DicomElement> = Vec::new();
        for element in elements {
            // Collect all the FileMeta elements to write them in one go, as their total byte
            // length is needed for the first element, FileMetaInformationGroupLength.
            if self.state == WriterState::WriteFileMeta {
                if element.tag() <= FILE_META_GROUP_END {
                    // Ignore FileMetaInformationGroupLength in place of one made below.
                    if element.tag() != FILE_META_INFORMATION_GROUP_LENGTH {
                        fm_elements.push(element);
                    }
                    continue;
                }

                bytes_written += self.write_owned_fm_elements(fm_elements.as_slice())?;
                // The list of FileMeta elements are no longer needed.
                fm_elements.clear();

                // Flip state to write standard elements, and fall-through. In the condition for
                // getting to this state the `element` value is non-FileMeta and hasn't been
                // written out yet.
                self.state = WriterState::WriteElement;
            }

            bytes_written += Writer::write_element(&mut self.dataset, self.ts, &element)?;
        }

        // If the input elements only consist of FileMeta elements then the above loop will never
        // result in writing any elements as they're being collected into `fm_elements`.
        if self.state == WriterState::WriteFileMeta && !fm_elements.is_empty() {
            bytes_written += self.write_owned_fm_elements(fm_elements.as_slice())?;
        }

        self.bytes_written += u64::try_from(bytes_written).unwrap_or_default();
        Ok(bytes_written)
    }

    /// Writes all the given `FileMeta` elements to an in-memory buffer, computes the length of the
    /// resulting bytes, and generates a `FileMetaInformationGroupLength` element, writes it to the
    /// dataset, then writes the in-memory buffer to the dataset as well.
    ///
    /// `fm_elements`: Slice of `&DicomElement`s which should all be elements with tag numbers in
    /// the range for `FileMeta`, and SHOULD NOT include a `FileMetaInformationGroupLength` element.
    fn write_fm_elements(&mut self, fm_elements: &[&DicomElement]) -> WriteResult<usize> {
        let mut bytes_written: usize = 0;
        let mut fm_dataset: Dataset<Vec<u8>> = Dataset::new(Vec::new());
        for fme in fm_elements {
            Writer::write_element(&mut fm_dataset, &ExplicitVRLittleEndian, fme)?;
        }
        let fm_bytes: Vec<u8> = fm_dataset.into_inner();

        let fm_group_length = Writer::<W>::new_fme(
            FILE_META_INFORMATION_GROUP_LENGTH,
            &vr::UL,
            RawValue::of_uint(u32::try_from(fm_bytes.len()).unwrap_or_default()),
        )?;

        bytes_written +=
            Writer::write_element(&mut self.dataset, &ExplicitVRLittleEndian, &fm_group_length)?;
        // The FileMeta elements have already been encoded, write the resulting bytes to
        // the Writer's dataset.
        self.dataset.write_all(&fm_bytes)?;
        bytes_written += fm_bytes.len();

        Ok(bytes_written)
    }

    /// Writes all the given `FileMeta` elements to an in-memory buffer, computes the length of the
    /// resulting bytes, and generates a `FileMetaInformationGroupLength` element, writes it to the
    /// dataset, then writes the in-memory buffer to the dataset as well.
    ///
    /// `fm_elements`: Slice of `&DicomElement`s which should all be elements with tag numbers in
    /// the range for `FileMeta`, and SHOULD NOT include a `FileMetaInformationGroupLength` element.
    ///
    /// # Notes
    /// Same as `write_fm_elements` but takes `&[DicomElement]`.
    fn write_owned_fm_elements(&mut self, fm_elements: &[DicomElement]) -> WriteResult<usize> {
        let fm_elem_refs = fm_elements.iter().collect::<Vec<&DicomElement>>();
        self.write_fm_elements(fm_elem_refs.as_slice())
    }

    fn new_fme(tag: u32, vr: VRRef, value: RawValue) -> WriteResult<DicomElement> {
        let mut element = DicomElement::new_empty(tag, vr, &ExplicitVRLittleEndian);

        element
            .encode_val(value)
            .map_err(<ParseError as Into<WriteError>>::into)?;

        Ok(element)
    }

    fn write_element(
        dataset: &mut Dataset<W>,
        dataset_ts: TSRef,
        element: &DicomElement,
    ) -> WriteResult<usize> {
        let mut bytes_written: usize = 0;

        // If the element is a sequence-delimiter, or is within a private sequence, then the
        // elements should be written with Implicit VR.
        let ts = if is_sq_delim(element.tag()) || is_parent_priv_sq(element.sq_path()) {
            if dataset_ts.big_endian() {
                &ImplicitVRBigEndian
            } else {
                &ImplicitVRLittleEndian
            }
        } else {
            dataset_ts
        };

        // TODO: Make this behavior a preference on the writer.
        // TODO: Performance improvements:
        //       - Some VRs are encoded with limited characters that will always be single-byte
        //         when using the default character set, and should not require re-encoding when
        //         the byte-endian changes.

        // Non-PixelData elements are only affected by change to byte-endian, e.g. a change
        // from ExplicitVRLittleEndian to JPEGBaselineProcess1 would only result in a change to
        // how PixelData tags are encoded but not any other elements.
        let non_pd_same_endian =
            !element.is_pixel_data() && element.ts().big_endian() == ts.big_endian();
        // Values without bytes and values encoded as plain bytes are not affected by change to
        // byte-endian.
        let non_endian_affected = element.is_empty() || element.vr() == &OB;

        let changed = if element.ts() == ts {
            None
        } else if non_pd_same_endian || non_endian_affected {
            let data = element.data().clone();
            let updated_elem = DicomElement::new(
                element.tag(),
                element.vr(),
                element.vl(),
                ts,
                element.cs(),
                data,
                element.sq_path().clone(),
            );
            Some(updated_elem)
        } else {
            // Re-encode the value to the new transfer syntax.
            let mut re_enc = DicomElement::new_empty(element.tag(), element.vr(), ts);
            let value = element.parse_value()?;
            re_enc.encode_val(value)?;
            Some(re_enc)
        };
        let element = changed.as_ref().unwrap_or(element);

        bytes_written += Writer::write_tag(dataset, element)?;
        bytes_written += Writer::write_vr(dataset, element)?;
        bytes_written += Writer::write_vl(dataset, element)?;
        bytes_written += Writer::write_data(dataset, element)?;

        Ok(bytes_written)
    }

    fn write_tag(dataset: &mut Dataset<W>, element: &DicomElement) -> WriteResult<usize> {
        let mut bytes_written: usize = 0;

        if element.ts().big_endian() {
            dataset.write_all(&u16::to_be_bytes(
                u16::try_from((element.tag() >> 16) & 0x0000_FFFF).unwrap_or(u16::MAX),
            ))?;
            bytes_written += std::mem::size_of::<u16>();

            dataset.write_all(&u16::to_be_bytes(
                u16::try_from(element.tag() & 0x0000_FFFF).unwrap_or(u16::MAX),
            ))?;
            bytes_written += std::mem::size_of::<u16>();
        } else {
            dataset.write_all(&u16::to_le_bytes(
                u16::try_from((element.tag() >> 16) & 0x0000_FFFF).unwrap_or(u16::MAX),
            ))?;
            bytes_written += std::mem::size_of::<u16>();
            dataset.write_all(&u16::to_le_bytes(
                u16::try_from(element.tag() & 0x0000_FFFF).unwrap_or(u16::MAX),
            ))?;
            bytes_written += std::mem::size_of::<u16>();
        }

        Ok(bytes_written)
    }

    /// Writes the VR to the dataset, if the transfer syntax requires explicit VR. If the transfer
    /// syntax requires implicit VR then nothing is written to the dataset.
    fn write_vr(dataset: &mut Dataset<W>, element: &DicomElement) -> WriteResult<usize> {
        if !element.ts().explicit_vr() {
            return Ok(0);
        }

        let mut bytes_written = 0usize;

        let vr = element.vr().ident;
        dataset.write_all(vr.as_bytes())?;
        bytes_written += vr.len();

        // When using Explicit VR and the VR specifies a 2byte padding then write out 16bits of
        // zeroes after the VR.
        // See Part 5, Ch 7.1.2
        if element.vr().has_explicit_2byte_pad {
            let padding = [0u8; 2];
            dataset.write_all(&padding)?;
            bytes_written += padding.len();
        }

        Ok(bytes_written)
    }

    fn write_vl(dataset: &mut Dataset<W>, element: &DicomElement) -> WriteResult<usize> {
        let mut bytes_written: usize = 0;

        let write_as_u32: bool = !element.ts().explicit_vr() || element.vr().has_explicit_2byte_pad;

        match element.vl() {
            ValueLength::UndefinedLength => {
                if !write_as_u32 {
                    return Err(WriteError::InvalidValueLength);
                }

                if element.ts().big_endian() {
                    dataset.write_all(&UNDEFINED_LENGTH.to_be_bytes())?;
                } else {
                    dataset.write_all(&UNDEFINED_LENGTH.to_le_bytes())?;
                }
                bytes_written += std::mem::size_of::<u32>();
            }

            ValueLength::Explicit(length) => {
                if write_as_u32 {
                    if element.ts().big_endian() {
                        dataset.write_all(&length.to_be_bytes())?;
                    } else {
                        dataset.write_all(&length.to_le_bytes())?;
                    }
                    bytes_written += std::mem::size_of::<u32>();
                } else {
                    let length: u16 = u16::try_from(length & 0x0000_FFFF).unwrap_or(u16::MAX);
                    if element.ts().big_endian() {
                        dataset.write_all(&length.to_be_bytes())?;
                    } else {
                        dataset.write_all(&length.to_le_bytes())?;
                    }
                    bytes_written += std::mem::size_of::<u16>();
                }
            }
        }

        Ok(bytes_written)
    }

    fn write_data(dataset: &mut Dataset<W>, element: &DicomElement) -> WriteResult<usize> {
        if element.data().is_empty() {
            return Ok(0);
        }

        #[cfg(feature = "compress")]
        {
            dataset.set_write_deflated(element.ts().deflated());
        }

        let mut bytes_written: usize = 0;
        dataset.write_all(element.data().as_slice())?;
        bytes_written += element.data().len();
        Ok(bytes_written)
    }
}
