use std::io::Read;

use super::error::ParseError;
use super::parser::Parser;
use super::parser::ParseResult;
use crate::core::dcmelement::DicomElement;

/// The implementation for `Parser` which is the core iteration loop.
impl<'dict, DatasetType: Read> Iterator for Parser<'dict, DatasetType> {
    type Item = ParseResult<DicomElement>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        // Once an error occurs, or the first `None` is returned then do not
        // continue trying to parse, and always return `None`.
        if self.iterator_ended {
            return None;
        }

        match self.iterate() {
            Err(ParseError::ExpectedEOF) => {
                self.iterator_ended = true;
                None
            }
            Err(e) => {
                self.iterator_ended = true;
                let detail = self.get_current_debug_str();
                Some(Err(ParseError::DetailedError {
                    source: Box::new(e),
                    detail,
                }))
            }
            Ok(None) => {
                self.iterator_ended = true;
                None
            }
            Ok(Some(element)) => Some(Ok(element)),
        }
    }
}
