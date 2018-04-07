// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::fmt;
use std::fs:: { OpenOptions, File as OutputFile };
use std::convert::{ AsRef };
use std::io:: { Result as IOResult };
use std::io::prelude::*;
use std::path::Path;
use report::summary:: { Summary };
use report::file:: { File, Files };
use record:: { RecordWrite };

pub mod attribute;
pub mod summary;
pub mod file;
pub mod branch;
pub mod line;
pub mod function;
pub mod test;
pub mod counter;

#[derive(Debug)]
pub struct Report {
    files: Files
}

impl Report {
    pub fn new(files: Files) -> Self {
        Report {
            files: files
        }
    }
    pub fn get(&self, key: &str) -> Option<&File> {
        self.files.get(&key.to_string())
    }
    pub fn files(&self) -> &Files {
        &self.files
    }
    pub fn len(&self) -> usize {
        self.files.len()
    }
    pub fn save_as<T: AsRef<Path>>(&self, path: T) -> IOResult<()> {
        let mut output = OpenOptions::new().create(true).write(true).open(path)?;
        self.write_records::<OutputFile>(&mut output)
    }
}

impl RecordWrite for Report {
    fn write_records<T: Write>(&self, output: &mut T) -> IOResult<()> {
        writeln!(output, "{}", self)
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (source_name, file) in self.files.iter() {
            for (test_name, test) in file.tests().iter() {
                writeln!(f, "TN:{}", test_name)?;
                writeln!(f, "SF:{}", source_name)?;
                write!(f, "{}", test.functions())?;
                write!(f, "{}", test.branches())?;
                write!(f, "{}", test.lines())?;
                writeln!(f, "end_of_record")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate tempdir;

    use self::tempdir::TempDir;
    use record::{ LineData, FunctionData, BranchData };
    use report::test::{ Tests };
    use report::file;
    use report::{ Report };
    use merger::ops:: { TryMerge };
    use std::fs::File;
    use std::io::*;

    fn build_report() -> Report {
        let mut tests = Tests::new();
        let line_data = &LineData { line: 1, count: 1, checksum: None };
        let function_data = &FunctionData { name: "main".to_string(), count: 1 };
        let branch_data = &BranchData { line: 1, block: 1, branch: 1, taken: 1 };
        let test_name = "test1".to_string();

        tests.try_merge((&test_name, line_data)).unwrap();
        tests.try_merge((&test_name, function_data)).unwrap();
        tests.try_merge((&test_name, branch_data)).unwrap();

        let file = file::File::new(tests);
        let mut files = file::Files::new();
        files.try_merge((&"a.c".to_string(), &file)).unwrap();

        Report::new(files)
    }

    #[test]
    fn save_as() {
        let report = build_report();
        let tmp_dir = TempDir::new("report").expect("create temp dir");
        let file_path = tmp_dir.path().join("report.lcov");
        let _ = report.save_as(file_path.clone()).unwrap();

        assert_eq!(file_path.as_path().exists(), true);
    }

    #[test]
    fn display() {
        let report = build_report();
        let report_path = "tests/fixtures/report/report.info";
        let readed_file_content = {
            let mut output = String::new();
            let mut f = File::open(report_path).unwrap();
            let _ = f.read_to_string(&mut output);
            output
        };
        assert_eq!(report.to_string(), readed_file_content);
    }
}
