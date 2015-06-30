//! Parser of LCOV report.

use record:: { LCOVRecord };
use combinator:: { record, records };
use lines::linereader:: { LineReader };
use nom:: { IResult };
use std::io:: { Read, Error, ErrorKind, Result };
use std::error::Error as ErrorDescription;
use std::str::{ from_utf8 };

#[derive(Debug, Clone)]
pub struct RecordError {
    line: u32,
    record: String,
    description: String
}

impl RecordError {
    fn new(line: &u32, record: &[u8], error: &Error) -> Self {
        RecordError {
            line: line.clone(),
            record: from_utf8(record).unwrap().to_string(),
            description: error.description().to_string()
        }
    }
}

/// Parser of LCOV report.
///
/// # Examples
///
/// ```
/// use lcov_parser:: { LCOVRecord, LCOVParser, RecordError };
/// use std::io:: { Error };
/// use std::fs::File;
///
/// struct TestParser {
///     records: Vec<LCOVRecord>,
///     record_errors: Vec<RecordError>
/// }
///
/// impl TestParser {
///     fn new() -> Self {
///         TestParser { records: vec!(), record_errors: vec!() }
///     }
/// }
///
/// impl LCOVParser for TestParser {
///     fn complete(&mut self, result: &LCOVRecord) {
///         self.records.push(result.clone())
///     }
///     fn failed(&mut self, error: &RecordError) {
///         self.record_errors.push(error.clone())
///     }
///     fn error(&mut self, error: &Error) {
///         println!("{:?}", error);
///     }
/// }
///
/// let f = File::open("./fixture/report.lcov").unwrap();
/// let mut parser = TestParser::new();
///
/// parser.parse(&f);
///
/// assert_eq!(parser.records.len(), 1);
/// ```
pub trait LCOVParser {
    fn parse<R: Read>(&mut self, reader: R) {
        let mut line_number = 0;
        let mut lr = LineReader::new(reader);

        loop {
            match lr.read_line() {
                Ok(b) if b.is_empty() => { break; },
                Ok(ref line) => {
                    line_number = line_number + 1;
                    self.parse_record(&line_number, line)
                },
                Err(ref e) => self.error(e)
            };
        }
    }
    fn parse_record(&mut self, line_number: &u32, line: &[u8]) {
        match parse_record(line) {
            Ok(ref record) => self.complete(record),
            Err(ref error) => self.failed( &RecordError::new(line_number, line, error))
        }
    }
    fn complete(&mut self, rc: &LCOVRecord);
    fn failed(&mut self, error: &RecordError);
    fn error(&mut self, error: &Error);
}

/// parse the record
///
/// # Examples
///
/// ```
/// use lcov_parser:: { LCOVRecord, parse_record };
///
/// let result = parse_record(b"TN:test_name\n");
///
/// assert_eq!(result.unwrap(), LCOVRecord::TestName("test_name".to_string()));
/// ```
pub fn parse_record(input: &[u8]) -> Result<LCOVRecord> {
    match record(input) {
        IResult::Done(_, record) => Ok(record),
        _ => Err(Error::new(ErrorKind::InvalidInput, "The record of file that can not be parsed."))
    }
}

/// parse all the records
///
/// # Examples
///
/// ```
/// use lcov_parser:: { LCOVRecord, parse_all_records };
///
/// let result = parse_all_records(b"TN:test_name\n");
///
/// assert_eq!(result.unwrap(), vec!( LCOVRecord::TestName("test_name".to_string())));
/// ```
pub fn parse_all_records(input: &[u8]) -> Result<Vec<LCOVRecord>> {
    match records(input) {
        IResult::Done(_, output) => Ok(output),
        _ => Err(Error::new(ErrorKind::InvalidInput, "The record of file that can not be parsed."))
    }
}
