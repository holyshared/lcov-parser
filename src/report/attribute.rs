// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use counter::Hit;

pub type TestName = String;
pub type SourceFile = String;
pub type LineNumber = u32;
pub type ExecutionCount = u32;
pub type FunctionName = String;
pub type CheckSum = String;

impl Hit for ExecutionCount {
    fn is_hit(&self) -> bool {
        *self > 0
    }
}
