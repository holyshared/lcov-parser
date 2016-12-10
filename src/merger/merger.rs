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
            try!(self.process_file(file));
        }
        Ok(Report::new(self.files.clone()))
    }
    fn process_file<T: AsRef<Path>>(&mut self, file: T) -> Result<(), MergeError> {
        let mut parser = try!(LCOVParser::from_file(file));

        loop {
            let result = try!(parser.next());

            if result.is_none() {
                break;
            }
            let record = result.unwrap();

            match record {
                LCOVRecord::TestName(ref name) => self.on_test_name(name),
                LCOVRecord::SourceFile(ref name) => self.on_source_file(name),
                LCOVRecord::Data(ref data) => try!(self.on_data(data)),
                LCOVRecord::FunctionName(ref func_name) => try!(self.on_func_name(func_name)),
                LCOVRecord::FunctionData(ref func_data) => try!(self.on_func_data(func_data)),
                LCOVRecord::BranchData(ref branch_data) => try!(self.on_branch_data(branch_data)),
                LCOVRecord::EndOfRecord => try!(self.on_end_of_record()),
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
            try!(self.tests.try_merge((test_name, line_data)));
        }
        Ok(())
    }
    fn on_func_name(&mut self, func_name: &FunctionName) -> MergeResult<FunctionError> {
        if self.test_name.is_none() {
            return Ok(());
        }

        let test_name = self.test_name.as_ref().unwrap();
        try!(self.tests.try_merge((test_name, func_name)));
        Ok(())
    }
    fn on_func_data(&mut self, func_data: &FunctionDataRecord) -> MergeResult<FunctionError> {
        if self.test_name.is_none() {
            return Ok(());
        }

        let test_name = self.test_name.as_ref().unwrap();
        try!(self.tests.try_merge((test_name, func_data)));
        Ok(())
    }
    fn on_branch_data(&mut self, branch_data: &BranchDataRecord) -> MergeResult<BranchError> {
        if self.test_name.is_none() {
            return Ok(());
        }
        let test_name = self.test_name.as_ref().unwrap();
        try!(self.tests.try_merge((test_name, branch_data)));
        Ok(())
    }
    fn on_end_of_record(&mut self) -> MergeResult<TestError> {
        let source_name = self.source_name.as_ref().unwrap();
        let file = File::new(self.tests.clone());

        try!(self.files.try_merge((source_name, &file)));
        self.tests = Tests::new();
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use merger::*;
    use merger::ops:: { MergeError, TestError, ChecksumError, MergeLine };
    use std::path::Path;
    use std::fs::File;
    use std::io::*;

    #[test]
    fn save_as() {
        let report_path = "tests/fixtures/merge/fixture.info";

        let mut parse = ReportMerger::new();
        let report = parse.merge(&[ report_path ]).unwrap();
        let _ = report.save_as("/tmp/report.lcov").unwrap();

        assert_eq!(Path::new("/tmp/report.lcov").exists(), true);
    }

    #[test]
    fn merge_checksum_error() {
        let report_path = "tests/fixtures/merge/without_checksum_fixture.info";
        let mut parse = ReportMerger::new();
        let result = parse.merge(&[ report_path, report_path ]).unwrap_err();

        let checksum_error = ChecksumError::Empty(MergeLine {
            line: 6,
            checksum: None
        });
        let test_error = TestError::from(checksum_error);

        // see pull request
        // https://github.com/rust-lang/rust/pull/34192
        assert!(match result {
            MergeError::Process(err) => err == test_error,
            _ => false
        })
    }

    #[test]
    fn display() {
        let report_path = "tests/fixtures/merge/fixture.info";
        let readed_file_content = {
            let mut output = String::new();
            let mut f = File::open(report_path).unwrap();
            let _ = f.read_to_string(&mut output);
            output
        };
        let mut parse = ReportMerger::new();
        let report = parse.merge(&[ report_path ]).unwrap();

        assert_eq!(report.to_string(), readed_file_content);
    }
}
