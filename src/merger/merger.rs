// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::path::Path;
use std::convert::{ AsRef };
use std::result:: { Result };
use parser:: { LCOVParser, FromFile };
use record:: { LCOVRecord, LineData, FunctionData as FunctionDataRecord, BranchData as BranchDataRecord, FunctionName };
use report:: { Report };
use report::test:: { Tests };
use report::file:: { File, Files };
use merger::ops:: { Merge, TryMerge, MergeError, TestError, ChecksumError, BranchError, FunctionError, MergeResult };

pub struct ReportMerger {
    test_name: Option<String>,
    source_name: Option<String>,
    tests: Tests,
    files: Files
}

impl ReportMerger {
    pub fn new() -> Self {
        ReportMerger {
            test_name: None,
            source_name: None,
            tests: Tests::new(),
            files: Files::new()
        }
    }
    pub fn merge<T: AsRef<Path>>(&mut self, files: &[T]) -> Result<Report, MergeError> {
        for file in files.iter() {
            self.process_file(file)?;
        }
        Ok(Report::new(self.files.clone()))
    }
    fn process_file<T: AsRef<Path>>(&mut self, file: T) -> Result<(), MergeError> {
        let mut parser = LCOVParser::from_file(file)?;

        loop {
            let result = parser.next()?;

            if result.is_none() {
                break;
            }
            let record = result.unwrap();

            match record {
                LCOVRecord::TestName(ref name) => self.on_test_name(name),
                LCOVRecord::SourceFile(ref name) => self.on_source_file(name),
                LCOVRecord::Data(ref data) => self.on_data(data)?,
                LCOVRecord::FunctionName(ref func_name) => self.on_func_name(func_name)?,
                LCOVRecord::FunctionData(ref func_data) => self.on_func_data(func_data)?,
                LCOVRecord::BranchData(ref branch_data) => self.on_branch_data(branch_data)?,
                LCOVRecord::EndOfRecord => self.on_end_of_record()?,
                _ => { continue; }
            };
        }
        Ok(())
    }

    fn on_test_name(&mut self, test_name: &Option<String>) {
        self.test_name = match test_name {
            &Some(ref name) => Some(name.clone()),
            &None => Some(String::new())
        };
        let current_test_name = self.test_name.as_ref().unwrap();
        self.tests.merge(current_test_name);
    }
    fn on_source_file(&mut self, source_name: &String) {
        self.source_name = Some(source_name.clone());
    }
    fn on_data(&mut self, line_data: &LineData) -> MergeResult<ChecksumError> {
        if self.test_name.is_some() {
            let test_name = self.test_name.as_ref().unwrap();
            self.tests.try_merge((test_name, line_data))?;
        }
        Ok(())
    }
    fn on_func_name(&mut self, func_name: &FunctionName) -> MergeResult<FunctionError> {
        if self.test_name.is_none() {
            return Ok(());
        }

        let test_name = self.test_name.as_ref().unwrap();
        self.tests.try_merge((test_name, func_name))?;
        Ok(())
    }
    fn on_func_data(&mut self, func_data: &FunctionDataRecord) -> MergeResult<FunctionError> {
        if self.test_name.is_none() {
            return Ok(());
        }

        let test_name = self.test_name.as_ref().unwrap();
        self.tests.try_merge((test_name, func_data))?;
        Ok(())
    }
    fn on_branch_data(&mut self, branch_data: &BranchDataRecord) -> MergeResult<BranchError> {
        if self.test_name.is_none() {
            return Ok(());
        }
        let test_name = self.test_name.as_ref().unwrap();
        self.tests.try_merge((test_name, branch_data))?;
        Ok(())
    }
    fn on_end_of_record(&mut self) -> MergeResult<TestError> {
        let source_name = self.source_name.as_ref().unwrap();
        let file = File::new(self.tests.clone());

        self.files.try_merge((source_name, &file))?;
        self.tests = Tests::new();
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use merger::*;
    use merger::ops:: { MergeError, TestError, ChecksumError, MergeLine };
    use report::summary::{ Summary };

    #[test]
    fn merge_checksum() {
        let report = {
            let report_path1 = "tests/fixtures/merged/eq_checksum/fixture1.info";
            let report_path2 = "tests/fixtures/merged/eq_checksum/fixture2.info";

            let mut parse = ReportMerger::new();
            parse.merge(&[ report_path1, report_path2 ]).unwrap()
        };

        let file = report.get("/fixture1.c").unwrap();
        let test = file.get_test(&"example".to_string()).unwrap();
        let lines = test.lines();
        let line = lines.get(&6).unwrap();

        assert_eq!(line.execution_count(), &2);
        assert_eq!(line.checksum(), Some(&"PF4Rz2r7RTliO9u6bZ7h6g".to_string()));
    }

    #[test]
    fn merge_checksum_one_side() {
        let check_merged_report = |report: Report| {
            let file = report.get("/fixture1.c").unwrap();
            let test = file.get_test(&"example".to_string()).unwrap();
            let lines = test.lines();
            let line = lines.get(&4).unwrap();

            assert_eq!(line.execution_count(), &2);
            assert_eq!(line.checksum(), Some(&"y7GE3Y4FyXCeXcrtqgSVzw".to_string()));
        };

        // report order: a -> b
        let report1 = {
            let report_path1 = "tests/fixtures/merged/one_side_checksum/fixture1.info";
            let report_path2 = "tests/fixtures/merged/one_side_checksum/fixture2.info";

            let mut parse = ReportMerger::new();
            parse.merge(&[ report_path1, report_path2 ]).unwrap()
        };
        check_merged_report(report1);

        // report order: b -> a
        let report2 = {
            let report_path1 = "tests/fixtures/merged/one_side_checksum/fixture1.info";
            let report_path2 = "tests/fixtures/merged/one_side_checksum/fixture2.info";

            let mut parse = ReportMerger::new();
            parse.merge(&[ report_path2, report_path1 ]).unwrap()
        };
        check_merged_report(report2);
    }

    #[test]
    #[ignore]
    fn merge_checksum_error() {
        let result = {
            let report_path1 = "tests/fixtures/merged/ne_checksum/fixture1.info";
            let report_path2 = "tests/fixtures/merged/ne_checksum/fixture2.info";

            let mut parse = ReportMerger::new();
            parse.merge(&[ report_path1, report_path2 ]).unwrap_err()
        };

        let checksum_error = ChecksumError::Mismatch(
            MergeLine::new(4, Some("y7GE3Y4FyXCeXcrtqgSVzw".to_string())),
            MergeLine::new(4, Some("invalid".to_string()))
        );
        let test_error = TestError::from(checksum_error);

        // see pull request
        // https://github.com/rust-lang/rust/pull/34192
        assert!(match result {
            MergeError::Process(err) => {
                println!("raised error: {}", err);
                err == test_error
            },
            _ => false
        })
    }
}
