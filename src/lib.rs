#[macro_use]
extern crate nom;
extern crate lines;

#[derive(Debug, PartialEq)]
pub enum LcovRecord
{
    TestName { name: String },
    SourceFile { file_name: String },
    Data { line_number: u32, executed_count: u32 },
    EndOfRecord
}

mod combinator;
pub mod parser;
