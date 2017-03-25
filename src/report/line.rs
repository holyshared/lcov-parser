// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::io;
use std::cmp::PartialEq;
use std::collections::btree_map:: { BTreeMap };
use std::convert::{ AsRef, From };
use std::fmt:: { Display, Formatter, Result };
use record:: { LineData, RecordWrite };
use report::summary:: { Summary };
use report::attribute:: { LineNumber, CheckSum, ExecutionCount };
use report::counter:: { Hit, HitFoundCounter, FoundCounter, HitCounter };
use merger::ops:: { TryMerge, MergeResult, MergeLine, ChecksumError };

#[derive(Debug, Eq, Clone)]
pub struct Line {
    line_number: LineNumber,
    execution_count: ExecutionCount,
    checksum: Option<CheckSum>
}

impl Line {
    pub fn new(
        line_number: LineNumber,
        execution_count: ExecutionCount,
        checksum: Option<CheckSum>
    ) -> Self {
        Line {
            line_number: line_number,
            execution_count: execution_count,
            checksum: checksum
        }
    }
    pub fn line_number(&self) -> &LineNumber {
        &self.line_number
    }
    pub fn execution_count(&self) -> &ExecutionCount {
        &self.execution_count
    }
    pub fn checksum(&self) -> Option<&CheckSum> {
        match self.checksum {
            Some(ref v) => Some(v),
            None => None
        }
    }
    pub fn has_checkshum(&self) -> bool {
        self.checksum.is_some()
    }
    pub fn is_hit(&self) -> bool {
        self.execution_count.is_hit()
    }
}

impl<'a> From<&'a LineData> for Line {
    fn from(line_data: &'a LineData) -> Self {
        Line::new(
            line_data.line,
            line_data.count,
            line_data.checksum.clone()
        )
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        let has_checkshum = self.has_checkshum() && other.has_checkshum();
        if has_checkshum {
            return self.checksum.as_ref() == other.checksum();
        }
        return &self.line_number == other.line_number();
    }
}

impl<'a> TryMerge<&'a Line> for Line {
    type Err = ChecksumError;

    fn try_merge(&mut self, other: &'a Line) -> MergeResult<Self::Err> {
        if !other.has_checkshum() {
            return Err(ChecksumError::Empty(MergeLine::from(other)));
        }

        if self.checksum.as_ref() != other.checksum() {
            return Err(ChecksumError::Mismatch(
                MergeLine::from(&self.clone()),
                MergeLine::from(other)
            ));
        }
        self.execution_count += *other.execution_count();
        Ok(())
    }
}

impl<'a> TryMerge<&'a LineData> for Line {
    type Err = ChecksumError;

    fn try_merge(&mut self, other: &'a LineData) -> MergeResult<Self::Err> {
        self.try_merge(&Line::from(other))
    }
}


#[derive(Debug, Clone)]
pub struct Lines {
    lines: BTreeMap<LineNumber, Line>
}

impl Lines {
    pub fn new() -> Self {
        Lines {
            lines: BTreeMap::new()
        }
    }
}

impl AsRef<BTreeMap<LineNumber, Line>> for Lines {
    fn as_ref(&self) -> &BTreeMap<LineNumber, Line> {
        &self.lines
    }
}

impl_summary!(Lines, lines<LineNumber, Line>);


impl HitCounter for Lines {
    fn hit_count(&self) -> usize {
        self.iter()
            .filter(|&(_, line)| line.is_hit() )
            .count()
    }
}

impl FoundCounter for Lines {
    fn found_count(&self) -> usize {
        self.lines.len()
    }
}

impl HitFoundCounter for Lines {
}



impl RecordWrite for Lines {
    fn write_records<T: io::Write>(&self, output: &mut T) -> io::Result<()> {
        write!(output, "{}", self)
    }
}

impl Display for Lines {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.is_empty() {
            return Ok(());
        }
        for (_, line) in self.iter() {
            match line.checksum() {
                Some(ref checksum) => writeln!(f, "DA:{},{},{}", line.line_number(), line.execution_count(), checksum)?,
                None => writeln!(f, "DA:{},{}", line.line_number(), line.execution_count())?
            }
        }
        writeln!(f, "LF:{}", self.found_count())?;
        writeln!(f, "LH:{}", self.hit_count())?;
        Ok(())
    }
}


impl<'a> TryMerge<&'a LineData> for Lines {
    type Err = ChecksumError;

    fn try_merge(&mut self, line_data: &'a LineData) -> MergeResult<Self::Err> {
        if !self.lines.contains_key(&line_data.line) {
            self.lines.insert(line_data.line, Line::from(line_data));
            return Ok(());
        }
        let mut line = self.lines.get_mut(&line_data.line).unwrap();
        line.try_merge(line_data)
    }
}


impl_try_merge_self_summary!(Lines:lines, ChecksumError);


#[cfg(test)]
mod tests {
    use merger::ops::*;
    use record:: { LineData };
    use report::line:: { Line, Lines };
    use report::summary:: { Summary };
    use report::counter:: { FoundCounter, HitCounter };

    #[test]
    fn add_line_data() {
        let mut lines = Lines::new();
        lines.try_merge(&LineData { line: 1, count: 1, checksum: Some("abc".to_string()) }).unwrap();
        lines.try_merge(&LineData { line: 1, count: 1, checksum: Some("abc".to_string()) }).unwrap();

        let result = lines.clone();
        assert_eq!( result.get(&1), Some(&Line::new(1, 2, Some("abc".to_string()))) );
    }

    #[test]
    fn add_lines_data() {
        let mut lines = Lines::new();
        lines.try_merge(&LineData { line: 1, count: 1, checksum: Some("abc".to_string()) }).unwrap();

        let ref cloned_lines = lines.clone();
        lines.try_merge(cloned_lines).unwrap();

        assert_eq!( lines.get(&1), Some(&Line::new(1, 2, Some("abc".to_string()))) );
    }

    #[test]
    fn hit_count_and_found_count() {
        let mut lines = Lines::new();

        lines.try_merge(&LineData { line: 1, count: 1, checksum: Some("abc".to_string()) }).unwrap();
        lines.try_merge(&LineData { line: 2, count: 0, checksum: Some("def".to_string()) }).unwrap();

        assert_eq!( lines.hit_count(), 1 );
        assert_eq!( lines.found_count(), 2 );
    }
}
