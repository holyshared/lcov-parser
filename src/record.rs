use combinator;
use nom:: { IResult };
use std::io:: { Error, ErrorKind, Result };

#[derive(Debug, PartialEq, Clone)]
pub enum LcovRecord
{
    TestName { name: String },
    SourceFile { file_name: String },
    Data { line_number: u32, executed_count: u32 },
    EndOfRecord
}

impl LcovRecord {
    pub fn record_from(input : &[u8]) -> Result<Self> {
        match combinator::record(input) {
            IResult::Done(_, record) => Ok(record),
            _ => Err(Error::new(ErrorKind::InvalidInput, "The record of file that can not be parsed."))
        }
    }
}
