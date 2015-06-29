use std::io:: { Result };
use parser:: { parse_record };

#[derive(Debug, PartialEq, Clone)]
pub enum LCOVRecord
{
    TestName { name: String },
    SourceFile { file_name: String },
    Data { line_number: u32, executed_count: u32 },
    EndOfRecord
}

impl LCOVRecord {

    /// Parse the record of data.
    ///
    /// # Examples
    ///
    /// ```
    /// use lcov_parser:: { LCOVRecord };
    ///
    /// let actual = LCOVRecord::record_from(b"TN:product_test\n").unwrap();
    /// let expected = LCOVRecord::TestName { name: "product_test".to_string() };
    ///
    /// assert_eq!(actual, expected);
    /// ```
    pub fn record_from(input : &[u8]) -> Result<Self> {
        parse_record(input)
    }
}
