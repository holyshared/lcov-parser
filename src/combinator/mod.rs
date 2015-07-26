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

use parser_combinators:: { many1, digit, string, satisfy, optional, token, value, try, between, newline, parser, Parser, ParserExt, ParseResult };
use parser_combinators::primitives:: { State, Stream };
use std::string:: { String };
use record:: { LCOVRecord, Token };

mod value;
mod branch;
mod function;
mod line;

use combinator::value:: { integer_value, string_value };
use combinator::branch:: { branch };
use combinator::function:: { function };
use combinator::line:: { lines_record };

#[inline]
pub fn record<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    parser(end_of_record::<I>)
        .or(parser(test_name::<I>))
        .or(parser(source_file::<I>))
        .or(parser(data::<I>))
        .or(parser(function::<I>))
        .or(parser(lines_record::<I>))
        .or(parser(branch::<I>))
        .parse_state(input)
}

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
/*
#[inline]
fn function_name<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let line_number = parser(integer_value::<I>);
    let function_name = token(',').with( parser(string_value::<I>) );

    let record = (line_number, function_name).map( | t | {
        let (line_number, function_name) = t;
        LCOVRecord::FunctionName(line_number, function_name)
    });
    between(string("FN:"), newline(), record).parse_state(input)
}

#[inline]
fn function_data<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let execution_count = parser(integer_value::<I>);
    let function_name = token(',')
        .with( parser(string_value::<I>) );

    let record = (execution_count, function_name).map( | t | {
        let (execution_count, function_name) = t;
        LCOVRecord::FunctionData(execution_count, function_name)
    });
    between(string("FNDA:"), newline(), record).parse_state(input)
}

#[inline]
fn functions_found<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let functions_found = parser(integer_value::<I>)
        .map( | functions_found | LCOVRecord::FunctionsFound(functions_found) );
    between(string("FNF:"), newline(), functions_found).parse_state(input)
}

#[inline]
fn functions_hit<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let functions_hit = parser(integer_value::<I>)
        .map( | functions_hit | LCOVRecord::FunctionsHit(functions_hit) );
    between(string("FNH:"), newline(), functions_hit).parse_state(input)
}
*/


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
/*
#[inline]
fn branch_data<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let line_number = parser(integer_value::<I>);
    let block_number = token(',').with( parser(integer_value::<I>) );
    let branch_number = token(',').with( parser(integer_value::<I>) );

    let called = parser(integer_value::<I>)
        .map(Token::Called);
    let not_called = token('-')
        .with( value(Token::NotCalled) );
    let branch_execution_count = try(not_called)
        .or(called);

    let taken = token(',').with(branch_execution_count);

    let record = (line_number, block_number, branch_number, taken).map( | t | {
        let (line_number, block_number, branch_number, taken) = t;
        LCOVRecord::BranchData(line_number, block_number, branch_number, taken)
    });
    between(string("BRDA:"), newline(), record).parse_state(input)
}

#[inline]
fn branches_found<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let branches_found = parser(integer_value::<I>)
        .map( | branches_found | LCOVRecord::BranchesFound(branches_found) );

    between(string("BRF:"), newline(), branches_found).parse_state(input)
}

#[inline]
fn branches_hit<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let branches_hit = parser(integer_value::<I>)
        .map( | branches_hit | LCOVRecord::BranchesHit(branches_hit) );

    between(string("BRH:"), newline(), branches_hit).parse_state(input)
}
*/

#[inline]
fn end_of_record<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    between(string("end_of_record"), newline(), value(LCOVRecord::EndOfRecord)).parse_state(input)
}

/*
#[inline]
fn integer_value<I>(input: State<I>) -> ParseResult<u32, I> where I: Stream<Item=char> {
    many1( digit() )
        .map( |s: String| s.parse::<u32>().unwrap() )
        .parse_state(input)
}

#[inline]
fn string_value<I>(input: State<I>) -> ParseResult<String, I> where I: Stream<Item=char> {
    many1( satisfy( | c: char | c != '\n' ) )
        .map( | s: String | s )
        .parse_state(input)
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
