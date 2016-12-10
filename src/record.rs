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
use std::io:: { Result };
use std::io::prelude::*;
use parser:: { parse_record };

#[derive(Debug, PartialEq, Clone)]
pub enum LCOVRecord
{
    TestName(Option<String>),         // TN:<test name>
    SourceFile(String),               // SF:<absolute path to the source file>
    Data(LineData),                   // DA:<line number>,<execution count>[,<checksum>]
    FunctionName(FunctionName),       // FN:<line number of function start>,<function name> for each function
    FunctionData(FunctionData),       // FNDA:<execution count>,<function name>
    FunctionsFound(u32),              // FNF:<number of functions found>
    FunctionsHit(u32),                // FNH:<number of function hit>
    LinesHit(u32),                    // LH:<number of lines with an execution count> greater than 0
    LinesFound(u32),                  // LF:<number of instrumented lines>
    BranchData(BranchData),           // BRDA:<line number>,<block number>,<branch number>,<taken>
    BranchesFound(u32),               // BRF:<number of branches found>
    BranchesHit(u32),                 // BRH:<number of branches hit>
    EndOfRecord                       // end_of_record
}

#[derive(Debug, PartialEq, Clone)]
pub struct LineData {
    pub line: u32,
    pub count: u32,
    pub checksum: Option<String> // MD5
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionName {
    pub name: String,
    pub line: u32
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionData {
    pub name: String,
    pub count: u32
}

#[derive(Debug, PartialEq, Clone)]
pub struct BranchData {
    pub line: u32,
    pub block: u32,
    pub branch: u32,
    pub taken: u32
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

impl From<LineData> for LCOVRecord {
    fn from(input: LineData) -> Self {
        LCOVRecord::Data(input)
    }
}

impl From<FunctionName> for LCOVRecord {
    fn from(input: FunctionName) -> Self {
        LCOVRecord::FunctionName(input)
    }
}

impl From<FunctionData> for LCOVRecord {
    fn from(input: FunctionData) -> Self {
        LCOVRecord::FunctionData(input)
    }
}

impl From<BranchData> for LCOVRecord {
    fn from(input: BranchData) -> Self {
        LCOVRecord::BranchData(input)
    }
}

pub trait RecordWrite {
    fn write_records<T: Write>(&self, output: &mut T) -> Result<()>;
}
