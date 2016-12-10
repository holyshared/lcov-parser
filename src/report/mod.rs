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
        let mut output = try!(OpenOptions::new().create(true).write(true).open(path));
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
                try!(writeln!(f, "TN:{}", test_name));
                try!(writeln!(f, "SF:{}", source_name));
                try!(write!(f, "{}", test.functions()));
                try!(write!(f, "{}", test.branches()));
                try!(write!(f, "{}", test.lines()));
                try!(writeln!(f, "end_of_record"));
            }
        }
        Ok(())
    }
}
