// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! The parser combinators for record.
//!
//! Supported record types are as follows.
//! Please see the following page for [the format](http://ltp.sourceforge.net/coverage/lcov/geninfo.1.php).
//!
//! * TN:<test name>
//! * SF:<absolute path to the source file>
//! * FN:<line number of function start>,<function name> for each function
//! * FNDA:<execution count>,<function name>
//! * FNF:<number of functions found>
//! * FNH:<number of function hit>
//! * DA:<line number>,<execution count>[,<checksum>]
//! * LH:<number of lines with an execution count> greater than 0
//! * LF:<number of instrumented lines>
//! * BRF:<number of branches found>
//! * BRH:<number of branches hit>
//! * end_of_record

use combine:: { parser, many, Parser, ParseResult, State, Stream };
use record:: { LCOVRecord  };

mod value;
mod general;
mod branch;
mod function;
mod line;

use combinator::general:: { general_record };
use combinator::branch:: { branch_record };
use combinator::function:: { function_record };
use combinator::line:: { lines_record };

#[inline]
pub fn report<I>(input: State<I>) -> ParseResult<Vec<LCOVRecord>, State<I>> where I: Stream<Item=char> {
    let record_parser = parser(record::<I>);
    many(record_parser).parse_stream(input)
}

#[inline]
pub fn record<I>(input: State<I>) -> ParseResult<LCOVRecord, State<I>> where I: Stream<Item=char> {
    parser(general_record::<I>)
        .or(parser(function_record::<I>))
        .or(parser(lines_record::<I>))
        .or(parser(branch_record::<I>))
        .parse_stream(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine:: { parser, Parser, State };
    use record:: { LCOVRecord, LineData, FunctionName, FunctionData, BranchData };

    fn parse_record(input: &str) -> LCOVRecord {
        let input = State::new(input);
        let (result, _) = parser(record).parse(input).unwrap();
        result
    }

    #[test]
    fn test_name() {
        let with_testname = parse_record("TN:test_name\n");
        assert_eq!(with_testname, LCOVRecord::TestName(Some("test_name".to_string())));

        let without_testname = parse_record("TN:\n");
        assert_eq!(without_testname, LCOVRecord::TestName(None));
    }

    #[test]
    fn source_file() {
        let result = parse_record("SF:/path/to/source.rs\n");
        assert_eq!(result, LCOVRecord::SourceFile("/path/to/source.rs".to_string()));
    }

    #[test]
    fn data() {
        let result = parse_record("DA:1,2\n");
        let line = LineData { line: 1, count: 2, checksum: None };
        assert_eq!(result, LCOVRecord::Data(line));
    }

    #[test]
    fn data_with_checksum() {
        let result = parse_record("DA:1,2,3sdfjiji56\n");
        let line = LineData { line: 1, count: 2, checksum: Some("3sdfjiji56".to_string()) };
        assert_eq!(result, LCOVRecord::Data(line));
    }

    #[test]
    fn function_name() {
        let result = parse_record("FN:5,main\n");
        let func = FunctionName { name: "main".to_string(), line: 5 };
        assert_eq!(result, LCOVRecord::FunctionName(func));
    }

    #[test]
    fn function_data() {
        let result = parse_record("FNDA:5,main\n");
        let func_data = FunctionData { name: "main".to_string(), count: 5 };
        assert_eq!(result, LCOVRecord::FunctionData(func_data));
    }

    #[test]
    fn functions_found() {
        let result = parse_record("FNF:10\n");
        assert_eq!(result, LCOVRecord::FunctionsFound(10));
    }

    #[test]
    fn functions_hit() {
        let result = parse_record("FNH:10\n");
        assert_eq!(result, LCOVRecord::FunctionsHit(10));
    }

    #[test]
    fn lines_hit() {
        let result = parse_record("LH:5\n");
        assert_eq!(result, LCOVRecord::LinesHit(5));
    }

    #[test]
    fn lines_found() {
        let result = parse_record("LF:10\n");
        assert_eq!(result, LCOVRecord::LinesFound(10));
    }

    #[test]
    fn branch_data() {
        let result = parse_record("BRDA:1,2,3,-\n");
        let branch = BranchData { line: 1, block: 2, branch: 3, taken: 0 };
        assert_eq!(result, LCOVRecord::BranchData(branch));
    }

    #[test]
    fn branch_data_with_branch_times() {
        let result = parse_record("BRDA:1,2,3,4\n");
        let branch = BranchData { line: 1, block: 2, branch: 3, taken: 4 };
        assert_eq!(result, LCOVRecord::BranchData(branch));
    }

    #[test]
    fn branches_found() {
        let result = parse_record("BRF:10\n");
        assert_eq!(result, LCOVRecord::BranchesFound(10));
    }

    #[test]
    fn branches_hit() {
        let result = parse_record("BRH:10\n");
        assert_eq!(result, LCOVRecord::BranchesHit(10));
    }

    #[test]
    fn end_of_record() {
        let result = parse_record("end_of_record\n");
        assert_eq!(result, LCOVRecord::EndOfRecord);
    }
}
