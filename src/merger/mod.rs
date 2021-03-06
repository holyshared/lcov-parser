// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

mod merger;
pub mod ops;

pub use merger::merger:: { ReportMerger };

use std::path::Path;
use report::*;
use self::ops::MergeError;

/// Merge reports
///
/// # Examples
///
/// ```
/// use lcov_parser::merge_files;
///
/// let trace_files = [
///    "tests/fixtures/merge/fixture.info",
///    "tests/fixtures/merge/fixture.info"
/// ];
/// let _ = match merge_files(&trace_files) {
///     Ok(report) => {
///         let result = report.save_as("/tmp/merged_report.info");
///         match result {
///             Ok(_) => println!("saved"),
///             Err(err) => println!("{}", err)
///         }
///     },
///     Err(err) => println!("{}", err)
/// };
/// ```
pub fn merge_files<T: AsRef<Path>>(files: &[T]) -> Result<Report, MergeError> {
    let mut merger = ReportMerger::new();
    merger.merge(files)
}
