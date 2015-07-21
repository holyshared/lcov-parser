//! Module of LCOV Record.

use std::convert:: { From };
use std::vec:: { Vec };
use std::option:: { Option };
use parser:: { parse_record };

#[derive(Debug, PartialEq, Clone)]
pub enum LCOVRecord
{
    TestName(String),                 // TN:<test name>
    SourceFile(String),               // SF:<absolute path to the source file>
    Data(u32, u32, Option<String>),   // DA:<line number>,<execution count>[,<checksum>]
    FunctionName(u32, String),        // FN:<line number of function start>,<function name> for each function
    FunctionData(u32, String),        // FNDA:<execution count>,<function name>
    FunctionsFound(u32),              // FNF:<number of functions found>
    FunctionsHit(u32),                // FNH:<number of function hit>
    LinesHit(u32),                    // LH:<number of lines with an execution count> greater than 0
    LinesFound(u32),                  // LF:<number of instrumented lines>
    BranchData(u32, u32, u32, Token), // BRDA:<line number>,<block number>,<branch number>,<taken>
    BranchesFound(u32),               // BRF:<number of branches found>
    BranchesHit(u32),                 // BRH:<number of branches hit>
    EndOfRecord                       // end_of_record
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Called(u32),
    NotCalled
}

/// Parse the record from [u8].
///
/// # Examples
///
/// ```
/// use lcov_parser:: { LCOVRecord };
///
/// let actual = LCOVRecord::from("TN:product_test\n".as_bytes());
/// let expected = LCOVRecord::TestName("product_test".to_string());
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
/// let expected = LCOVRecord::TestName("product_test".to_string());
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
/// let expected = LCOVRecord::TestName("product_test".to_string());
///
/// assert_eq!(actual, expected);
/// ```
impl<'a> From<&'a Vec<u8>> for LCOVRecord {
    fn from(input: &'a Vec<u8>) -> Self {
        parse_record(&input[..]).unwrap()
    }
}
