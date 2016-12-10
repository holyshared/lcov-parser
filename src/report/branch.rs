// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::io;
use std::cmp::PartialEq;
use std::fmt:: { Display, Formatter, Result };
use std::collections::btree_map:: { BTreeMap };
use std::convert::{ From };
use record:: { BranchData };
use merger::ops:: { TryMerge, MergeResult, MergeBranch, BranchError };
use record:: { RecordWrite };
use report::summary:: { Summary };
use report::attribute:: { LineNumber, ExecutionCount };
use report::counter:: { Hit, HitFoundCounter, FoundCounter, HitCounter };

/// Units of the branch
///
/// # Examples
///
/// ```
/// use lcov_parser::branch::BranchUnit;
///
/// let branch1 = BranchUnit::new(1, 1);
/// let branch2 = BranchUnit::new(1, 1);
///
/// assert!(branch1 == branch2);
///
/// let not_eq_branch1 = BranchUnit::new(1, 1);
/// let not_eq_branch2 = BranchUnit::new(1, 2);
///
/// assert!(not_eq_branch1 != not_eq_branch2);
/// ```
#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct BranchUnit(u32, u32);

impl BranchUnit {
    pub fn new(block: u32, branch: u32) -> BranchUnit {
        BranchUnit(block, branch)
    }
    pub fn block(&self) -> &u32 {
        &self.0
    }
    pub fn branch(&self) -> &u32 {
        &self.1
    }
}

impl Display for BranchUnit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
pub struct Branch {
    line_number: LineNumber,
    block: u32,
    branch: u32,
    execution_count: ExecutionCount
}

impl Branch {
    pub fn new(
        line_number: LineNumber,
        block: u32,
        branch: u32,
        execution_count: ExecutionCount
    ) -> Self {
        Branch {
            line_number: line_number,
            block: block,
            branch: branch,
            execution_count: execution_count
        }
    }
    pub fn line_number(&self) -> &LineNumber {
        &self.line_number
    }
    pub fn block(&self) -> &u32 {
        &self.block
    }
    pub fn branch(&self) -> &u32 {
        &self.branch
    }
    pub fn execution_count(&self) -> &ExecutionCount {
        &self.execution_count
    }
}

impl PartialEq<BranchData> for Branch {
    fn eq(&self, data: &BranchData) -> bool {
        if self.line_number != data.line {
            return false;
        }
        let branch_matched = self.block == data.block && self.branch == data.branch;

        if !branch_matched {
            return false;
        }
        return true;
    }
}

impl PartialEq<Branch> for Branch {
    fn eq(&self, other: &Branch) -> bool {
        if self.line_number != *other.line_number() {
            return false;
        }
        let branch_matched = self.block == *other.block() && self.branch == *other.branch();

        if !branch_matched {
            return false;
        }
        return true;
    }
}







impl<'a> From<&'a BranchData> for Branch {
    fn from(data: &'a BranchData) -> Self {
        Branch::new(
            data.line,
            data.block,
            data.branch,
            data.taken
        )
    }
}

impl<'a> TryMerge<&'a BranchData> for Branch {
    type Err = BranchError;

    fn try_merge(&mut self, data: &'a BranchData) -> MergeResult<Self::Err> {
        if self != data {
            return Err(
                BranchError::Mismatch(
                    MergeBranch::from(&self.clone()),
                    MergeBranch::from(data)
                )
            );
        }
        self.execution_count += data.taken;
        Ok(())
    }
}

impl<'a> TryMerge<&'a Branch> for Branch {
    type Err = BranchError;

    fn try_merge(&mut self, other: &'a Branch) -> MergeResult<Self::Err> {
        if self != other {
            return Err(
                BranchError::Mismatch(
                    MergeBranch::from(&self.clone()),
                    MergeBranch::from(other)
                )
            );
        }
        self.execution_count += *other.execution_count();
        Ok(())
    }
}

impl Hit for Branch {
    fn is_hit(&self) -> bool {
        self.execution_count.is_hit()
    }
}




#[derive(Debug, PartialEq, Clone)]
pub struct BranchBlocks {
    blocks: BTreeMap<BranchUnit, Branch>
}

impl BranchBlocks {
    pub fn new() -> Self {
        BranchBlocks {
            blocks: BTreeMap::new()
        }
    }
}

impl_summary!(BranchBlocks, blocks<BranchUnit, Branch>);


impl HitCounter for BranchBlocks {
    fn hit_count(&self) -> usize {
        self.iter()
            .filter(|&(_, branch)| branch.is_hit())
            .count()
    }
}

impl FoundCounter for BranchBlocks {
    fn found_count(&self) -> usize {
        self.blocks.len()
    }
}

impl HitFoundCounter for BranchBlocks {}



impl<'a> TryMerge<&'a BranchData> for BranchBlocks {
    type Err = BranchError;

    fn try_merge(&mut self, data: &'a BranchData) -> MergeResult<Self::Err> {
        let unit = BranchUnit::new(data.block, data.branch);
        if !self.blocks.contains_key(&unit) {
            self.blocks.insert(unit, Branch::from(data));
            return Ok(());
        }
        let mut block = self.blocks.get_mut(&unit).unwrap();
        block.try_merge(data)
    }
}

impl_try_merge_self_summary!(BranchBlocks:blocks, BranchError);



#[derive(Debug, Clone)]
pub struct Branches {
    branches: BTreeMap<LineNumber, BranchBlocks>
}

impl Branches {
    pub fn new() -> Self {
        Branches {
            branches: BTreeMap::new()
        }
    }
}

impl HitCounter for Branches {
    fn hit_count(&self) -> usize {
        self.iter()
            .map(|(_, blocks)| blocks.hit_count() )
            .sum()
    }
}

impl FoundCounter for Branches {
    fn found_count(&self) -> usize {
        self.iter()
            .map(|(_, blocks)| blocks.found_count() )
            .sum()
    }
}

impl HitFoundCounter for Branches {}


impl_summary!(Branches, branches<LineNumber, BranchBlocks>);


impl RecordWrite for Branches {
    fn write_records<T: io::Write>(&self, output: &mut T) -> io::Result<()> {
        write!(output, "{}", self)
    }
}

impl Display for Branches {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.is_empty() {
            return Ok(());
        }
        for (line_number, blocks) in self.iter() {
            for (_, branch) in blocks.iter() {
                try!(writeln!(f, "BRDA:{},{},{},{}",
                    line_number, branch.block(), branch.branch(), branch.execution_count()));
            }
        }
        try!(writeln!(f, "BRF:{}", self.found_count()));
        try!(writeln!(f, "BRH:{}", self.hit_count()));
        Ok(())
    }
}

impl_try_merge_self_summary!(Branches:branches, BranchError);


impl<'a> TryMerge<&'a BranchData> for Branches {
    type Err = BranchError;

    fn try_merge(&mut self, data: &'a BranchData) -> MergeResult<Self::Err> {
        if self.branches.contains_key(&data.line) {
            let mut blocks = self.branches.get_mut(&data.line).unwrap();
            blocks.try_merge(data)
        } else {
            let blocks = {
                let mut blocks = BranchBlocks::new();
                let _ = try!(blocks.try_merge(data));
                blocks
            };
            self.branches.insert(
                data.line.clone(),
                blocks
            );
            Ok(())
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections:: { HashMap };
    use merger::ops::*;
    use record:: { BranchData };
    use report::branch:: { Branch, BranchUnit, Branches, BranchBlocks };
    use report::summary:: { Summary };
    use report::counter:: { FoundCounter, HitCounter };

    #[test]
    fn branch_unit() {
        let branch1 = BranchUnit(1, 1);
        let branch2 = BranchUnit(1, 2);

        assert!(branch1 != branch2);

        let same_branch1 = BranchUnit(1, 1);
        let same_branch2 = BranchUnit(1, 1);
    
        assert_eq!(same_branch1, same_branch2);
    }

    #[test]
    fn branch_unit_as_hash_key() {
        let mut container = HashMap::new();
        container.insert(BranchUnit(1, 1), 1);

        assert!( container.contains_key(&BranchUnit(1, 1)) );
    }

    #[test]
    fn add_branch_data() {
        let mut branches = BranchBlocks::new();
        let b1 = &BranchData { line: 1, block: 0, branch: 1, taken: 1 };
        let b2 = &BranchData { line: 1, block: 0, branch: 1, taken: 1 };

        branches.try_merge(b1).unwrap();
        branches.try_merge(b2).unwrap();

        let branch = Branch::new(1, 0, 1, 2);
        assert_eq!(branches.get(&BranchUnit::new(0, 1)), Some(&branch));
    }

    #[test]
    fn append_branches() {
        let mut branches = BranchBlocks::new();
        let b1 = &BranchData { line: 1, block: 0, branch: 1, taken: 1 };
        let b2 = &BranchData { line: 1, block: 0, branch: 1, taken: 1 };

        branches.try_merge(b1).unwrap();
        branches.try_merge(b2).unwrap();

        let cloned_branches = branches.clone();
        branches.try_merge(&cloned_branches).unwrap();

        let branch = Branch::new(1, 0, 1, 2);
        assert_eq!(branches.get(&BranchUnit::new(0, 1)), Some(&branch));
    }

    #[test]
    fn branch_blocks_hit_count_and_found_count() {
        let mut branches = BranchBlocks::new();
        let b1 = &BranchData { line: 1, block: 0, branch: 1, taken: 1 };
        let b2 = &BranchData { line: 1, block: 0, branch: 2, taken: 0 };

        branches.try_merge(b1).unwrap();
        branches.try_merge(b2).unwrap();

        assert_eq!(branches.hit_count(), 1);
        assert_eq!(branches.found_count(), 2);
    }

    #[test]
    fn branches_hit_count_and_found_count() {
        let mut branches = Branches::new();
        branches.try_merge(&BranchData { line: 1, block: 0, branch: 1, taken: 1 }).unwrap();
        branches.try_merge(&BranchData { line: 1, block: 0, branch: 2, taken: 0 }).unwrap();

        assert_eq!(branches.hit_count(), 1);
        assert_eq!(branches.found_count(), 2);
    }
}
