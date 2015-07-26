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

use parser_combinators:: { parser, Parser, ParserExt, ParseResult };
use parser_combinators::primitives:: { State, Stream };
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
pub fn record<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    parser(general_record::<I>)
        .or(parser(function_record::<I>))
        .or(parser(lines_record::<I>))
        .or(parser(branch_record::<I>))
        .parse_state(input)
}
/*
#[inline]
fn test_name<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let test_name = parser(string_value::<I>).map( | s: String | LCOVRecord::TestName(s) );
    between(string("TN:"), newline(), test_name).parse_state(input)
}

#[inline]
fn source_file<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let source_file =  parser(string_value::<I>).map( | s: String | LCOVRecord::SourceFile(s) );
    between(string("SF:"), newline(), source_file).parse_state(input)
}

#[inline]
fn data<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let line_number = parser(integer_value::<I>);
    let execution_count = token(',').with( parser(integer_value::<I>) );
    let checksum = optional( token(',').with( parser(string_value::<I>) ) );
    let record = (line_number, execution_count, checksum).map( | t | {
        let (line_number, execution_count, checksum) = t;
        LCOVRecord::Data(line_number, execution_count, checksum)
    });
    between(string("DA:"), newline(), record).parse_state(input)
}

#[inline]
fn end_of_record<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    between(string("end_of_record"), newline(), value(LCOVRecord::EndOfRecord)).parse_state(input)
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use parser_combinators:: { parser, Parser };
    use record:: { LCOVRecord, Token };

    #[test]
    fn test_name() {
        let result = parser(record).parse("TN:test_name\n");
        assert_eq!(result.unwrap(), (LCOVRecord::TestName("test_name".to_string()), ""));
    }

    #[test]
    fn source_file() {
        let result = parser(record).parse("SF:/path/to/source.rs\n");
        assert_eq!(result.unwrap(), (LCOVRecord::SourceFile("/path/to/source.rs".to_string()), ""));
    }

    #[test]
    fn data() {
        let result = parser(record).parse("DA:1,2\n");
        assert_eq!(result.unwrap(), (LCOVRecord::Data(1, 2, None), ""));
    }

    #[test]
    fn data_with_checksum() {
        let result = parser(record).parse("DA:1,2,3sdfjiji56\n");
        assert_eq!(result.unwrap(), (LCOVRecord::Data(1, 2, Some("3sdfjiji56".to_string())), ""));
    }

    #[test]
    fn function_name() {
        let result = parser(record).parse("FN:5,main\n");
        assert_eq!(result.unwrap(), (LCOVRecord::FunctionName(5, "main".to_string()), ""));
    }

    #[test]
    fn function_data() {
        let result = parser(record).parse("FNDA:5,main\n");
        assert_eq!(result.unwrap(), (LCOVRecord::FunctionData(5, "main".to_string()), ""));
    }

    #[test]
    fn functions_found() {
        let result = parser(record).parse("FNF:10\n");
        assert_eq!(result.unwrap(), (LCOVRecord::FunctionsFound(10), ""));
    }

    #[test]
    fn functions_hit() {
        let result = parser(record).parse("FNH:10\n");
        assert_eq!(result.unwrap(), (LCOVRecord::FunctionsHit(10), ""));
    }

    #[test]
    fn lines_hit() {
        let result = parser(record).parse("LH:5\n");
        assert_eq!(result.unwrap(), (LCOVRecord::LinesHit(5), ""));
    }

    #[test]
    fn lines_found() {
        let result = parser(record).parse("LF:10\n");
        assert_eq!(result.unwrap(), (LCOVRecord::LinesFound(10), ""));
    }

    #[test]
    fn branch_data() {
        let result = parser(record).parse("BRDA:1,2,3,-\n");
        assert_eq!(result.unwrap(), (LCOVRecord::BranchData(1, 2, 3, Token::NotCalled), ""));
    }

    #[test]
    fn branch_data_with_branch_times() {
        let result = parser(record).parse("BRDA:1,2,3,4\n");
        assert_eq!(result.unwrap(), (LCOVRecord::BranchData(1, 2, 3, Token::Called(4)), ""));
    }

    #[test]
    fn branches_found() {
        let result = parser(record).parse("BRF:10\n");
        assert_eq!(result.unwrap(), (LCOVRecord::BranchesFound(10), ""));
    }

    #[test]
    fn branches_hit() {
        let result = parser(record).parse("BRH:10\n");
        assert_eq!(result.unwrap(), (LCOVRecord::BranchesHit(10), ""));
    }

    #[test]
    fn end_of_record() {
        let result = parser(record).parse("end_of_record\n");
        assert_eq!(result.unwrap(), (LCOVRecord::EndOfRecord, ""));
    }
}
