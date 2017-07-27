// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::result::Result;
use std::convert::From;
use std::io:: { Error as IOError};
use std::fmt;
use parser:: { ParseError, RecordParseError };
use record:: { BranchData };
use report::line:: { Line };
use report::branch:: { Branch };
use report::attribute:: { LineNumber, FunctionName, CheckSum };

pub type MergeResult<E> = Result<(), E>;

pub trait Merge<Rhs=Self> {
    fn merge(&mut self, Rhs);
}

pub trait TryMerge<Rhs=Self> {
    type Err;
    fn try_merge(&mut self, Rhs) -> MergeResult<Self::Err>;
}

#[derive(Debug, PartialEq)]
pub enum ChecksumError {
    Empty(MergeLine),
    Mismatch(MergeLine, MergeLine)
}

impl fmt::Display for ChecksumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ChecksumError::Empty(ref line) => {
                write!(f, "No source code checksum: {}", line)
            },
            &ChecksumError::Mismatch(ref line1, ref line2) => {
                write!(f, "Source code checksums do not match: line: {}, left: {}, right: {}",
                    line1.line(),
                    line1.checksum(),
                    line2.checksum())
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MergeLine {
    line: LineNumber,
    checksum: Option<CheckSum>
}

impl MergeLine {
    pub fn new(line_number: LineNumber, checksum: Option<CheckSum>) -> MergeLine {
        MergeLine {
            line: line_number,
            checksum: checksum
        }
    }
    pub fn line(&self) -> &LineNumber {
        &self.line
    }
    pub fn checksum(&self) -> &str {
        match self.checksum {
            Some(ref checksum) => checksum.as_str(),
            None => &""
        }
    }
}

impl<'a> From<&'a Line> for MergeLine {
    fn from(line: &'a Line) -> Self {
        let line_number = line.line_number().clone();
        let checksum = match line.checksum() {
            Some(value) => Some(value.clone()),
            None => None
        };
        MergeLine::new(line_number, checksum)
    }
}

impl fmt::Display for MergeLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line: {}, checksum: {}", self.line(), self.checksum())
    }
}

#[derive(Debug, PartialEq)]
pub enum FunctionError {
    Mismatch(FunctionName, FunctionName)
}

impl fmt::Display for FunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &FunctionError::Mismatch(ref func1, ref func2) => {
                write!(f, "Function name mismatch: left = {}, right = {}", func1, func2)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MergeBranch {
    pub line: LineNumber,
    pub block: u32,
    pub branch: u32
}

impl<'a> From<&'a Branch> for MergeBranch {
    fn from(branch: &'a Branch) -> Self {
        let line = branch.line_number();
        let block = branch.block();
        let branch = branch.branch();
        MergeBranch {
            line: *line,
            block: *block,
            branch: *branch
        }
    }
}

impl<'a> From<&'a BranchData> for MergeBranch {
    fn from(branch: &'a BranchData) -> Self {
        MergeBranch {
            line: branch.line,
            block: branch.block,
            branch: branch.branch
        }
    }
}

impl fmt::Display for MergeBranch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line:{} block:{} branch:{}", self.line, self.block, self.branch)
    }
}

#[derive(Debug, PartialEq)]
pub enum BranchError {
    Mismatch(MergeBranch, MergeBranch)
}

impl fmt::Display for BranchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &BranchError::Mismatch(ref branch1, ref branch2) => {
                write!(f, "Branch mismatch: left = {}, right = {}", branch1, branch2)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TestError {
    Checksum(ChecksumError),
    Function(FunctionError),
    Branch(BranchError)
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &TestError::Checksum(ref err) => write!(f, "{}", err),
            &TestError::Function(ref err) => write!(f, "{}", err),
            &TestError::Branch(ref err) => write!(f, "{}", err)
        }
    }
}

impl_from_error!(ChecksumError, TestError::Checksum);
impl_from_error!(FunctionError, TestError::Function);
impl_from_error!(BranchError, TestError::Branch);

#[derive(Debug)]
pub enum MergeError {
    IO(IOError),
    RecordParse(RecordParseError),
    Process(TestError)
}

impl_from_error!(IOError, MergeError::IO);
impl_from_error!(ChecksumError, TestError::Checksum=>MergeError::Process);
impl_from_error!(FunctionError, TestError::Function=>MergeError::Process);
impl_from_error!(BranchError, TestError::Branch=>MergeError::Process);

impl From<ParseError> for MergeError {
    fn from(error: ParseError) -> Self {
        match error {
            ParseError::IOError(io) => MergeError::IO(io),
            ParseError::RecordParseError(record) => MergeError::RecordParse(record)
        }
    }
}

impl_from_error!(TestError, MergeError::Process);

impl fmt::Display for MergeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &MergeError::IO(ref err) => write!(f, "{}", err),
            &MergeError::RecordParse(ref err) => write!(f, "{}", err),
            &MergeError::Process(ref err) => write!(f, "{}", err)
        }
    }
}



#[cfg(test)]
mod tests {
    use merger::ops:: { MergeError, TestError, ChecksumError, MergeLine };

    #[test]
    fn merge_error_of_checksum_empty() {
        let merge_line = MergeLine::new(1, None);
        let checksum_error = ChecksumError::Empty(merge_line);
        let test_error = TestError::from(checksum_error);
        let merge_error = MergeError::from(test_error);
        assert_eq!(merge_error.to_string(), "No source code checksum: line: 1, checksum: ")
    }

    #[test]
    fn merge_error_of_checksum() {
        let merge_line1 = MergeLine::new(1, Some("xyz".to_string()));
        let merge_line2 = MergeLine::new(1, Some("zzz".to_string()));
        let checksum_error = ChecksumError::Mismatch(merge_line1, merge_line2);
        let test_error = TestError::from(checksum_error);
        let merge_error = MergeError::from(test_error);
        assert_eq!(merge_error.to_string(), "Source code checksums do not match: line: 1, left: xyz, right: zzz")
    }
}
