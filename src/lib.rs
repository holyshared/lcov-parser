#[macro_use]
extern crate nom;
extern crate lines;

//std::clone::Clone

#[derive(Debug, PartialEq, Clone)]
pub enum LcovRecord
{
    TestName { name: String },
    SourceFile { file_name: String },
    Data { line_number: u32, executed_count: u32 },
    EndOfRecord
}

#[derive(Debug, Clone)]
pub struct RecordError {
    line_number: u32,
    message: String,
    record: String
}

mod combinator;
pub mod parser;
