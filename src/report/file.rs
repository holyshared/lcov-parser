// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::collections::btree_map:: { BTreeMap };
use merger::ops:: { TryMerge, MergeResult, TestError };
use report::test:: { Test, Tests };
use report::summary:: { Summary };
use report::attribute:: { SourceFile };

#[derive(Debug, Clone)]
pub struct File {
    tests: Tests
}

impl File {
    pub fn new(tests: Tests) -> Self {
        File {
            tests: tests
        }
    }
    pub fn tests(&self) -> &Tests {
        &self.tests
    }
    pub fn get_test(&self, name: &String) -> Option<&Test> {
        self.tests.get(name)
    }
}

impl<'a> TryMerge<&'a File> for File {
    type Err = TestError;

    fn try_merge(&mut self, file: &'a File) -> MergeResult<Self::Err> {
        self.tests.try_merge(file.tests())
    }
}



#[derive(Debug, Clone)]
pub struct Files {
    files: BTreeMap<SourceFile, File>
}

impl Files {
    pub fn new() -> Self {
        Files {
            files: BTreeMap::new()
        }
    }
}

impl_summary!(Files, files<SourceFile, File>);


impl<'a> TryMerge<(&'a SourceFile, &'a File)> for Files {
    type Err = TestError;

    fn try_merge(&mut self, source_file: (&'a SourceFile, &'a File)) -> MergeResult<Self::Err> {
        if !self.files.contains_key(source_file.0) {
            self.files.insert(source_file.0.clone(), source_file.1.clone());
            return Ok(());
        }
        let file = self.files.get_mut(source_file.0).unwrap();
        file.try_merge(source_file.1)
    }
}

impl_try_merge_self_summary!(Files:files, TestError);
