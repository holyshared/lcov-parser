// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Parser of LCOV report.

use combine:: { parser, Parser, ParseError };
use combine::primitives:: { Stream };
use record:: { LCOVRecord };
use combinator:: { record, report };
use std::io:: { Read };
use std::fs:: { File };
use std::result:: { Result };
use std::path:: { Path };
use std::convert:: { From };

pub type ParseResult<T> = Result<T, RecordParseError>;

#[derive(PartialEq, Debug)]
pub struct RecordParseError {
    pub line: i32,
    pub column: i32
}

///
/// # Examples
///
/// ```
/// use lcov_parser:: { LCOVParser, LCOVRecord };
///
/// let res = LCOVParser::new("TN:testname\nSF:/path/to/source.rs\n").parse().unwrap();
///
/// assert_eq!(res[0], LCOVRecord::TestName("testname".to_string()));
/// assert_eq!(res[1], LCOVRecord::SourceFile("/path/to/source.rs".to_string()));
/// ```

pub struct LCOVParser {
    report: String
}

impl LCOVParser {
    pub fn new(report: &str) -> Self {
        LCOVParser { report: report.to_string() }
    }
    pub fn parse(&self) -> ParseResult<Vec<LCOVRecord>> {
        let value = self.report.as_str();
        let records = try!(parse_report(value));
        Ok(records)
    }
}

impl<P: AsRef<Path>> From<P> for LCOVParser {
    fn from(path: P) -> Self {
        let mut file = File::open(path).unwrap();
        let mut buffer = String::new();
        let _ = file.read_to_string(&mut buffer);
        LCOVParser::new(buffer.as_str())
    }
}

impl<P: Stream<Item=char>> From<ParseError<P>> for RecordParseError {
    fn from(error: ParseError<P>) -> Self {
        let line = error.position.line;
        let column = error.position.column;
        RecordParseError { line: line, column: column }
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
pub fn parse_record(input: &str) -> ParseResult<LCOVRecord> {
    let (record, _) = try!(parser(record).parse(input));
    Ok(record)
}

#[inline]
pub fn parse_report(input: &str) -> ParseResult<Vec<LCOVRecord>> {
    let (records, _) = try!(parser(report).parse(input));
    Ok(records)
}
