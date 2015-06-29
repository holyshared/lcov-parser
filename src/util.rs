//! Utility of parser
//!
//! provides functions to support the parse.

use record:: { LCOVRecord };
use parser:: { parse_all_records };
use std::ops:: { Fn };

/// processes the records in order
///
/// # Examples
///
/// ```
/// use lcov_parser::util:: { each_records };
///
/// each_records(b"TN:test_name\n", |r| println!("{:?}", r))
/// ```
pub fn each_records<F>(input: &[u8], callback: F)
    where F : Fn(&LCOVRecord) {

    match parse_all_records(input) {
        Ok(records) => {
            for record in records.iter() {
                callback(&record)
            }
        },
        Err(error) => panic!("{:?}", error)
    }
}
