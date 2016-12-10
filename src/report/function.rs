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
use std::convert::{ From };
use std::fmt:: { Display, Formatter, Result };
use record:: { FunctionName as FunctionNameRecord, FunctionData };
use merger::ops:: { TryMerge, MergeResult, FunctionError };
use record:: { RecordWrite };
use report::summary:: { Summary };
use report::attribute:: { ExecutionCount, FunctionName, LineNumber };
use report::counter:: { Hit, HitFoundCounter, FoundCounter, HitCounter };


#[derive(Debug, Clone)]
pub struct Functions {
    functions: BTreeMap<FunctionName, Function>
}

impl Functions {
    pub fn new() -> Self {
        Functions {
            functions: BTreeMap::new()
        }
    }
}

impl_summary!(Functions, functions<FunctionName, Function>);

impl HitCounter for Functions {
    fn hit_count(&self) -> usize {
        self.iter()
            .filter(|&(_, function)| function.is_hit() )
            .count()
    }
}

impl FoundCounter for Functions {
    fn found_count(&self) -> usize {
        self.functions.len()
    }
}

impl HitFoundCounter for Functions {
}


impl RecordWrite for Functions {
    fn write_records<T: io::Write>(&self, output: &mut T) -> io::Result<()> {
        write!(output, "{}", self)
    }
}

impl Display for Functions {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.is_empty() {
            return Ok(());
        }
        for (_, function) in self.iter() {
            try!(writeln!(f, "FN:{},{}", function.line_number(), function.name()));
            try!(writeln!(f, "FNDA:{},{}", function.execution_count(), function.name()));
        }
        try!(writeln!(f, "FNF:{}", self.hit_count()));
        try!(writeln!(f, "FNH:{}", self.found_count()));
        Ok(())
    }
}



impl<'a> TryMerge<&'a FunctionData> for Functions {
    type Err = FunctionError;

    fn try_merge(&mut self, function_data: &'a FunctionData) -> MergeResult<Self::Err> {
        if !self.functions.contains_key(&function_data.name) {
            self.functions.insert(
                function_data.name.clone(),
                Function::from(function_data)
            );
            return Ok(());
        }
        let mut function = self.functions.get_mut(&function_data.name).unwrap();
        function.try_merge(function_data)
    }
}


impl<'a> TryMerge<&'a FunctionNameRecord> for Functions {
    type Err = FunctionError;

    fn try_merge(&mut self, function_name: &'a FunctionNameRecord) -> MergeResult<Self::Err> {
        if !self.functions.contains_key(&function_name.name) {
            self.functions.insert(
                function_name.name.clone(),
                Function::from(function_name)
            );
            return Ok(());
        }
        let mut function = self.functions.get_mut(&function_name.name).unwrap();
        function.try_merge(function_name)
    }
}

impl_try_merge_self_summary!(Functions:functions, FunctionError);


#[derive(Debug, Clone)]
pub struct Function {
    name: FunctionName,
    line_number: LineNumber,
    execution_count: ExecutionCount
}

impl Function {
    pub fn new(
        name: FunctionName,
        line_number: LineNumber,
        execution_count: ExecutionCount,
    ) -> Self {
        Function {
            name: name,
            line_number: line_number,
            execution_count: execution_count
        }
    }
    pub fn name(&self) -> &FunctionName {
        &self.name
    }
    pub fn line_number(&self) -> &LineNumber {
        &self.line_number
    }
    pub fn execution_count(&self) -> &ExecutionCount {
        &self.execution_count
    }
    pub fn is_hit(&self) -> bool {
        self.execution_count.is_hit()
    }
}

impl<'a> From<&'a FunctionData> for Function {
    fn from(function_data: &'a FunctionData) -> Self {
        Function::new(
            function_data.name.clone(),
            0,
            function_data.count
        )
    }
}


impl<'a> From<&'a FunctionNameRecord> for Function {
    fn from(function_name: &'a FunctionNameRecord) -> Self {
        Function::new(
            function_name.name.clone(),
            function_name.line,
            0
        )
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        &self.name == other.name() && &self.line_number == other.line_number()
    }
}

impl<'a> TryMerge<&'a FunctionNameRecord> for Function {
    type Err = FunctionError;

    fn try_merge(&mut self, other: &'a FunctionNameRecord) -> MergeResult<Self::Err> {
        if self.name != other.name {
            return Err(FunctionError::Mismatch(
                self.name.clone(),
                other.name.clone()
            ));
        }
        self.name = other.name.clone();
        self.line_number = other.line;
        Ok(())
    }
}



impl<'a> TryMerge<&'a FunctionData> for Function {
    type Err = FunctionError;

    fn try_merge(&mut self, other: &'a FunctionData) -> MergeResult<Self::Err> {
        if self.name != other.name {
            return Err(FunctionError::Mismatch(
                self.name.clone(),
                other.name.clone()
            ));
        }
        self.execution_count += other.count;
        Ok(())
    }
}


impl<'a> TryMerge<&'a Function> for Function {
    type Err = FunctionError;

    fn try_merge(&mut self, other: &'a Function) -> MergeResult<Self::Err> {
        if self.name() != other.name() {
            return Err(FunctionError::Mismatch(
                self.name.clone(),
                other.name().clone()
            ));
        }
        self.execution_count += *other.execution_count();
        Ok(())
    }
}




#[cfg(test)]
mod tests {
    use record:: { FunctionData };
    use report::function:: { Function, Functions };
    use report::summary:: { Summary };
    use report::counter:: { FoundCounter, HitCounter };
    use merger::ops::*;

    #[test]
    fn add_function_data() {
        let mut functions = Functions::new();
        functions.try_merge(&FunctionData { name: "main".to_string(), count: 1 }).unwrap();
        functions.try_merge(&FunctionData { name: "main".to_string(), count: 1 }).unwrap();

        let result = functions.clone();
        assert_eq!( result.get(&"main".to_string()), Some( &Function::new("main".to_string(), 0, 1)));
    }

    #[test]
    fn add_functions_data() {
        let mut functions = Functions::new();
        functions.try_merge(&FunctionData { name: "main".to_string(), count: 1 }).unwrap();

        let ref cloned_functions = functions.clone();
        functions.try_merge(cloned_functions).unwrap();

        assert_eq!( functions.get(&"main".to_string()), Some( &Function::new("main".to_string(), 0, 2)));
    }

    #[test]
    fn hit_count_and_found_count() {
        let mut functions = Functions::new();
        functions.try_merge(&FunctionData { name: "main".to_string(), count: 1 }).unwrap();
        functions.try_merge(&FunctionData { name: "main".to_string(), count: 0 }).unwrap();
        functions.try_merge(&FunctionData { name: "foo".to_string(), count: 0 }).unwrap();

        assert_eq!( functions.hit_count(), 1 );
        assert_eq!( functions.found_count(), 2 );
    }
}
