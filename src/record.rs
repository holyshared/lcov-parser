//! Module of LCOV Record.

use std::convert:: { From };
use std::vec:: { Vec };
use parser:: { parse_record };

#[derive(Debug, PartialEq, Clone)]
pub enum LCOVRecord
{
    TestName { name: String },
    SourceFile { file_name: String },
    Data { line_number: u32, executed_count: u32 },
    EndOfRecord
}

/// Parse the record from [u8].
///
/// # Examples
///
/// ```
/// use lcov_parser:: { LCOVRecord };
///
/// let actual = LCOVRecord::from("TN:product_test\n".as_bytes());
/// let expected = LCOVRecord::TestName { name: "product_test".to_string() };
///
/// assert_eq!(actual, expected);
/// ```
impl<'a> From<&'a [u8]> for LCOVRecord {
    fn from(input: &'a [u8]) -> Self {
        parse_record(input).unwrap()
    }
}

/// Parse the record from &str.
///
/// # Examples
///
/// ```
/// use lcov_parser:: { LCOVRecord };
///
/// let actual = LCOVRecord::from("TN:product_test\n");
/// let expected = LCOVRecord::TestName { name: "product_test".to_string() };
///
/// assert_eq!(actual, expected);
/// ```
impl<'a> From<&'a str> for LCOVRecord {
    fn from(input: &'a str) -> Self {
        parse_record(input.as_bytes()).unwrap()
    }
}

/// Parse the record from Vec<u8>.
///
/// # Examples
///
/// ```
/// use lcov_parser:: { LCOVRecord };
///
/// let input: Vec<u8> = "TN:product_test\n".bytes().collect();
///
/// let actual = LCOVRecord::from(&input);
/// let expected = LCOVRecord::TestName { name: "product_test".to_string() };
///
/// assert_eq!(actual, expected);
/// ```
impl<'a> From<&'a Vec<u8>> for LCOVRecord {
    fn from(input: &'a Vec<u8>) -> Self {
        parse_record(&input[..]).unwrap()
    }
}
