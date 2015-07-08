//! Parser of LCOV report.

use parser_combinators:: { parser, Parser };
use record:: { LCOVRecord };
use combinator2:: { record, records };
use std::str:: { from_utf8 };
use std::io:: { Result, Error, ErrorKind };

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
            match parse_for_record(value) {
                Ok(record) => Ok(record),
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

#[inline]
fn parse_for_record(input: &str) -> Result<LCOVRecord> {
    let parsed_result = parser(record).parse(input);

    match parsed_result {
        Ok((record, _)) => Ok(record),
        Err(error) => Err(Error::new(ErrorKind::InvalidInput, format!("{}", error)))
    }
}
