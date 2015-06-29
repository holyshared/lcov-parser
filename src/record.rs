#[derive(Debug, PartialEq, Clone)]
pub enum LcovRecord
{
    TestName { name: String },
    SourceFile { file_name: String },
    Data { line_number: u32, executed_count: u32 },
    EndOfRecord
}
