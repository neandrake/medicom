use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};

use dcmpipe_lib::core::read::{Parser, ParserBuilder};
use dcmpipe_lib::core::read::stop::ParseStop;

pub struct MockDicomDataset {
    pub data: Vec<u8>,
    pub pos: usize,
}

impl MockDicomDataset {
    fn create_parser(
        mockup: MockDicomDataset,
        tagstop: ParseStop,
    ) -> Parser<'static, MockDicomDataset> {
        ParserBuilder::default().stop(tagstop).build(mockup)
    }

    pub fn standard_dicom_preamble() -> Parser<'static, MockDicomDataset> {
        let mockup: MockDicomDataset = MockDicomDataset {
            data: {
                let mut data: Vec<u8> = vec![0u8; 132];
                data[128] = 'D' as u8;
                data[129] = 'I' as u8;
                data[130] = 'C' as u8;
                data[131] = 'M' as u8;
                data
            },
            pos: 0,
        };
        MockDicomDataset::create_parser(mockup, ParseStop::EndOfDataset)
    }

    pub fn invalid_dicom_prefix() -> Parser<'static, MockDicomDataset> {
        let mockup: MockDicomDataset = MockDicomDataset {
            data: {
                let mut data: Vec<u8> = vec![0u8; 132];
                data[128] = 'D' as u8;
                data[129] = 'O' as u8;
                data[130] = 'C' as u8;
                data[131] = 'M' as u8;
                data
            },
            pos: 0,
        };
        MockDicomDataset::create_parser(mockup, ParseStop::EndOfDataset)
    }

    pub fn nonzero_preamble() -> Parser<'static, MockDicomDataset> {
        let mockup: MockDicomDataset = MockDicomDataset {
            data: {
                let mut data: Vec<u8> = vec![0xFFu8; 132];
                data[128] = 'D' as u8;
                data[129] = 'I' as u8;
                data[130] = 'C' as u8;
                data[131] = 'M' as u8;
                data
            },
            pos: 0,
        };
        MockDicomDataset::create_parser(mockup, ParseStop::EndOfDataset)
    }

    pub fn standard_dicom_preamble_diff_startpos_and_short_dataset(
    ) -> Parser<'static, MockDicomDataset> {
        let mockup: MockDicomDataset = MockDicomDataset {
            data: {
                let mut data: Vec<u8> = vec![0u8; 132];
                data[128] = 'D' as u8;
                data[129] = 'I' as u8;
                data[130] = 'C' as u8;
                data[131] = 'M' as u8;
                data
            },
            pos: 131,
        };
        MockDicomDataset::create_parser(mockup, ParseStop::EndOfDataset)
    }

    pub fn standard_dicom_header_bad_explicit_vr() -> Parser<'static, MockDicomDataset> {
        let mockup: MockDicomDataset = MockDicomDataset {
            data: vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x44, 0x49, 0x43, 0x4D, 0x02, 0x00, 0x00, 0x00, 0x55, 0x4C, 0x04, 0x00,
                0xE2, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x4F, 0x42, 0x00, 0x00, 0x02, 0x00,
                0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x02, 0x00, 0x55, 0x49, 0x1A, 0x00, 0x31, 0x2E,
                0x32, 0x2E, 0x38, 0x34, 0x30, 0x2E, 0x31, 0x30, 0x30, 0x30, 0x38, 0x2E, 0x35, 0x2E,
                0x31, 0x2E, 0x34, 0x2E, 0x31, 0x2E, 0x31, 0x2E, 0x32, 0x00, 0x02, 0x00, 0x03, 0x00,
                0x55, 0x49, 0x34, 0x00, 0x31, 0x2E, 0x32, 0x2E, 0x32, 0x37, 0x36, 0x2E, 0x30, 0x2E,
                0x37, 0x32, 0x33, 0x30, 0x30, 0x31, 0x30, 0x2E, 0x33, 0x2E, 0x31, 0x2E, 0x34, 0x2E,
                0x31, 0x37, 0x38, 0x37, 0x32, 0x30, 0x35, 0x34, 0x32, 0x38, 0x2E, 0x32, 0x33, 0x34,
                0x35, 0x2E, 0x31, 0x30, 0x37, 0x31, 0x30, 0x34, 0x38, 0x31, 0x34, 0x36, 0x2E, 0x31,
                0x02, 0x00, 0x10, 0x00, 0x55, 0x49, 0x14, 0x00, 0x31, 0x2E, 0x32, 0x2E, 0x38, 0x34,
                0x30, 0x2E, 0x31, 0x30, 0x30, 0x30, 0x38, 0x2E, 0x31, 0x2E, 0x32, 0x2E, 0x35, 0x00,
                0x02, 0x00, 0x12, 0x00, 0x55, 0x49, 0x30, 0x00, 0x31, 0x2E, 0x32, 0x2E, 0x38, 0x32,
                0x36, 0x2E, 0x30, 0x2E, 0x31, 0x2E, 0x33, 0x36, 0x38, 0x30, 0x30, 0x34, 0x33, 0x2E,
                0x32, 0x2E, 0x31, 0x31, 0x34, 0x33, 0x2E, 0x31, 0x30, 0x37, 0x2E, 0x31, 0x30, 0x34,
                0x2E, 0x31, 0x30, 0x33, 0x2E, 0x31, 0x31, 0x35, 0x2E, 0x32, 0x2E, 0x31, 0x2E, 0x30,
                0x02, 0x00, 0x13, 0x00, 0x53, 0x48, 0x0A, 0x00, 0x47, 0x44, 0x43, 0x4D, 0x20, 0x32,
                0x2E, 0x31, 0x2E, 0x30, 0x02, 0x00, 0x16, 0x00, 0x41, 0x45, 0x08, 0x00, 0x67, 0x64,
                0x63, 0x6D, 0x63, 0x6F, 0x6E, 0x76, 0x08, 0x00, 0x05, 0x00, 0x43, 0x53, 0x0A, 0x00,
                0x49, 0x53, 0x4F, 0x5F, 0x49, 0x52, 0x20, 0x31, 0x30, 0x30, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
            pos: 0,
        };
        MockDicomDataset::create_parser(mockup, ParseStop::EndOfDataset)
    }
}

impl Read for MockDicomDataset {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        if self.pos >= self.data.len() {
            return Result::Ok(0);
        }

        let mut count: usize = 0;
        for i in self.pos..self.data.len() {
            if count >= buf.len() {
                break;
            }
            buf[count] = self.data[i];
            count += 1;
        }
        self.pos = self.pos + count;
        Result::Ok(count)
    }
}

impl Seek for MockDicomDataset {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
        let newpos: usize = match pos {
            SeekFrom::Start(n) => 0usize.saturating_add(n as usize),
            SeekFrom::Current(n) => self.pos.saturating_add(n as usize),
            SeekFrom::End(n) => self.data.len().saturating_sub(n as usize),
        };

        if newpos < self.data.len() {
            self.pos = newpos;
            return Result::Ok(newpos as u64);
        }

        return Result::Err(Error::new(
            ErrorKind::UnexpectedEof,
            format!("seek to invalid position: {:?}", newpos),
        ));
    }
}
