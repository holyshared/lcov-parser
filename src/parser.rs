// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Parser of LCOV report.

use combine:: { parser, Parser };
use record:: { LCOVRecord };
use combinator:: { record, report };
use std::io:: { Read, ErrorKind };
use std::fs:: { File };
use std::result:: { Result };
use std::path:: { Path };
use std::ops:: { Fn };
use std::convert:: { From };

#[derive(PartialEq, Debug)]
pub enum ParsedResult {
    Ok(LCOVRecord, u32),
    Eof,
    Err(RecordParsedError)
}

#[derive(PartialEq, Debug)]
pub enum RecordParsedError {
    Read(ErrorKind),
    Record(String, i32)
}

///
/// # Examples
///
/// ```
/// use lcov_parser:: { ReportParser, LCOVRecord, ParsedResult };
///
/// let res = ReportParser::new("TN:testname\nSF:/path/to/source.rs\n").parse().unwrap();
///
/// assert_eq!(res[0], LCOVRecord::TestName("testname".to_string()));
/// assert_eq!(res[1], LCOVRecord::SourceFile("/path/to/source.rs".to_string()));
/// ```

pub struct ReportParser {
    report: String
}

impl ReportParser {
    pub fn new(report: &str) -> Self {
        ReportParser { report: report.to_string() }
    }
    pub fn parse(&self) -> Result<Vec<LCOVRecord>, RecordParsedError> {
        let value = self.report.as_str();
        match parse_report(value) {
            Ok(records) =>  Ok(records),
            Err(error) => Err(error)
        }
    }
}

impl<P: AsRef<Path>> From<P> for ReportParser {
    fn from(path: P) -> Self {
        let mut file = File::open(path).unwrap();
        let mut buffer = String::new();
        let _ = file.read_to_string(&mut buffer);
        ReportParser::new(buffer.as_str())
    }
}

/// parse the record
///
/// # Examples
///
/// ```
/// use lcov_parser:: { LCOVRecord, parse_record };
///
/// let result = parse_record("TN:test_name\n");
///
/// assert_eq!(result.unwrap(), LCOVRecord::TestName("test_name".to_string()));
/// ```

#[inline]
pub fn parse_record(input: &str) -> Result<LCOVRecord, RecordParsedError> {
    match parser(record).parse(input) {
        Ok((record, _)) => Ok(record),
        Err(error) => {
            let column = error.position.column;
            let source = input.to_string();
            Err( RecordParsedError::Record(source, column) )
        }
    }
}

#[inline]
pub fn parse_report(input: &str) -> Result<Vec<LCOVRecord>, RecordParsedError> {
    match parser(report).parse(input) {
        Ok((records, _)) => Ok(records),
        Err(error) => {
            let column = error.position.column;
            let source = input.to_string();
            Err( RecordParsedError::Record(source, column) )
        }
    }
}

/// processes the records in order
///
/// # Examples
///
/// ```
/// use lcov_parser:: { each_records };
///
/// each_records("TN:test_name\n", |r| println!("{:?}", r))
/// ```

#[inline]
pub fn each_records<F>(input: &str, callback: F)
    where F : Fn(LCOVRecord) {

    match parse_report(input) {
        Ok(records) => {
            for record in records {
                callback(record);
            }
        },
        Err(error) => panic!("{:?}", error)
    }
}
