//! Parser of LCOV report.

use parser_combinators:: { parser, Parser };
use lines::linereader:: { LineReader };
use record:: { LCOVRecord };
use combinator:: { record };
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
/// use lcov_parser:: { LCOVParser, LCOVRecord, ParsedResult };
///
/// let mut parser = LCOVParser::new("TN:testname\nSF:/path/to/source.rs\n".as_bytes());
/// let res1 = parser.parse_next();
/// let res2 = parser.parse_next();
///
/// assert_eq!(res1, ParsedResult::Ok(LCOVRecord::TestName("testname".to_string()), 1));
/// assert_eq!(res2, ParsedResult::Ok(LCOVRecord::SourceFile("/path/to/source.rs".to_string()), 2));
/// ```

pub struct LCOVParser<R> {
    line: u32,
    reader: LineReader<R>
}

impl<R: Read> LCOVParser<R> {
    pub fn new(reader: R) -> Self {
        LCOVParser { reader: LineReader::new(reader), line: 0 }
    }
    pub fn parse_next(&mut self) -> ParsedResult {
        match self.reader.read_line() {
            Ok(b) if b.is_empty() => ParsedResult::Eof,
            Ok(input) => {
                self.line = self.line + 1;
                match parse_record(input) {
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
/// use lcov_parser:: { LCOVRecord, parse_record };
///
/// let result = parse_record(b"TN:test_name\n");
///
/// assert_eq!(result.unwrap(), LCOVRecord::TestName("test_name".to_string()));
/// ```

#[inline]
pub fn parse_record(input: &[u8]) -> Result<LCOVRecord> {
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
