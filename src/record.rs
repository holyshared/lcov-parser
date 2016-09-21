// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Module of LCOV Record.

use std::convert:: { From };
use std::option:: { Option };
use parser:: { parse_record };

#[derive(Debug, PartialEq, Clone)]
pub enum LCOVRecord
{
    TestName(Option<String>),         // TN:<test name>
    SourceFile(String),               // SF:<absolute path to the source file>
    Data(u32, u32, Option<String>),   // DA:<line number>,<execution count>[,<checksum>]
    FunctionName(u32, String),        // FN:<line number of function start>,<function name> for each function
    FunctionData(u32, String),        // FNDA:<execution count>,<function name>
    FunctionsFound(u32),              // FNF:<number of functions found>
    FunctionsHit(u32),                // FNH:<number of function hit>
    LinesHit(u32),                    // LH:<number of lines with an execution count> greater than 0
    LinesFound(u32),                  // LF:<number of instrumented lines>
    BranchData(u32, u32, u32, u32),   // BRDA:<line number>,<block number>,<branch number>,<taken>
    BranchesFound(u32),               // BRF:<number of branches found>
    BranchesHit(u32),                 // BRH:<number of branches hit>
    EndOfRecord                       // end_of_record
}

/// Parse the record from &str.
///
/// # Examples
///
/// ```
/// use lcov_parser:: { LCOVRecord };
///
/// let actual = LCOVRecord::from("TN:product_test\n");
/// let expected = LCOVRecord::TestName(Some("product_test".to_string()));
///
/// assert_eq!(actual, expected);
/// ```
impl<'a> From<&'a str> for LCOVRecord {
    fn from(input: &'a str) -> Self {
        parse_record(input).unwrap()
    }
}
