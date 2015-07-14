//! Parser of LCOV report.

use parser_combinators:: { parser, Parser };
use lines::linereader:: { LineReader };
use record:: { LCOVRecord };
use combinator2:: { record, records };
use std::str:: { from_utf8 };
use std::io:: { Read, Result, Error, ErrorKind };

#[derive(PartialEq, Debug)]
pub struct ParsedError(String);

#[derive(PartialEq, Debug)]
pub enum ParsedResult {
    Ok(LCOVRecord, u32),
    Eof,
    Err(ParsedError)
}


///
/// # Examples
///
/// ```
/// use std::io:: { Read };
/// use lcov_parser:: { LCOVParser2, LCOVRecord, ParsedResult };
///
/// let mut parser = LCOVParser2::new("TN:testname\n".as_bytes());
/// let res1 = parser.parse_next();
///
/// assert_eq!(res1, ParsedResult::Ok(LCOVRecord::TestName("testname".to_string()), 1));
/// ```

pub struct LCOVParser2<R> {
    line: u32,
    reader: LineReader<R>
}

impl<R: Read> LCOVParser2<R> {
    pub fn new(reader: R) -> Self {
        LCOVParser2 { reader: LineReader::new(reader), line: 0 }
    }
    pub fn parse_next(&mut self) -> ParsedResult {
        match self.reader.read_line() {
            Ok(b) if b.is_empty() => ParsedResult::Eof,
            Ok(input) => {
                self.line = self.line + 1;
                match parse_record2(input) {
                    Ok(record) => ParsedResult::Ok(record, self.line),
                    Err(_) => ParsedResult::Err(ParsedError("a".to_string()))
                }
            },
            Err(_) => ParsedResult::Err(ParsedError("a".to_string()))
        }
    }
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
