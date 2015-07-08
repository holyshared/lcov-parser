//! Parser of LCOV report.

use parser_combinators:: { parser, Parser };
use lines::linereader:: { LineReader };
use record:: { LCOVRecord };
use combinator2:: { record, records };
use std::str:: { from_utf8 };
use std::io:: { Read, Result, Error, ErrorKind };

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedResult {
    line: u32,
    record: LCOVRecord
}

impl ParsedResult {
    fn new(line: &u32, record: &LCOVRecord) -> ParsedResult {
        ParsedResult { line: line.clone(), record: record.clone() }
    }
}

pub trait LCOVParser2 {
    fn parse<R: Read>(&mut self, reader: R) {
        let mut line = 0;
        let mut lr = LineReader::new(reader);

        loop {
            match lr.read_line() {
                Ok(b) if b.is_empty() => { break; },
                Ok(input) => {
                    line = line + 1;
                    self.parse_record(&line, &input);
                },
                Err(e) => self.error(&e)
            };
        }
    }
    fn parse_record(&mut self, line: &u32, input: &[u8]) {
        match parse_record2(input) {
            Ok(ref record) => self.complete( &ParsedResult::new(line, record) ),
            Err(ref error) => self.failed(error)
        }
    }
    fn complete(&mut self, record: &ParsedResult);
    fn failed(&mut self, error: &Error);
    fn error(&mut self, error: &Error);
}

/// parse the record
///
/// # Examples
///
/// ```
/// use lcov_parser:: { LCOVRecord, parse_record2 };
///
/// let result = parse_record2(b"TN:test_name\n");
///
/// assert_eq!(result.unwrap(), LCOVRecord::TestName("test_name".to_string()));
/// ```

#[inline]
pub fn parse_record2(input: &[u8]) -> Result<LCOVRecord> {
    match from_utf8(input) {
        Ok(value) => {
            match parser(record).parse(value) {
                Ok((record, _)) => Ok(record),
                Err(error) => Err(Error::new(ErrorKind::InvalidInput, format!("{}", error)))
            }
        },
        Err(error) => Err(Error::new(ErrorKind::InvalidInput, format!("{}", error)))
    }
}

/// parse all the records
///
/// # Examples
///
/// ```
/// use lcov_parser:: { LCOVRecord, parse_all_records2 };
///
/// let result = parse_all_records2(b"TN:test_name\n");
///
/// assert_eq!(result.unwrap(), vec!( LCOVRecord::TestName("test_name".to_string())));
/// ```

#[inline]
pub fn parse_all_records2(input: &[u8]) -> Result<Vec<LCOVRecord>> {
    match from_utf8(input) {
        Ok(value) => {
            match parser(records).parse(value) {
                Ok((records, _)) => Ok(records),
                Err(error) => Err(Error::new(ErrorKind::InvalidInput, format!("{}", error)))
            }
        },
        Err(error) => Err(Error::new(ErrorKind::InvalidInput, format!("{}", error)))
    }
}
