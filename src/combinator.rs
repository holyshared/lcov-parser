//! The parser combinators for record.
//!
//! Supported record types are as follows.
//! Please see the following page for [the format](http://ltp.sourceforge.net/coverage/lcov/geninfo.1.php).
//!
//! * TN:<test name>
//! * SF:<absolute path to the source file>
//! * DA:<line number>,<execution count>[,<checksum>]
//! * end_of_record

use parser_combinators:: { many1, digit, string, satisfy, optional, token, value, try, between, newline, parser, Parser, ParserExt, ParseResult };
use parser_combinators::primitives:: { State, Stream };
use std::string:: { String };
use record:: { LCOVRecord };

#[inline]
pub fn record<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    parser(end_of_record::<I>)
        .or(parser(test_name::<I>))
        .or(parser(source_file::<I>))
        .or(parser(data::<I>))
        .or(parser(function_name::<I>))
        .or(try(parser(lines_hit::<I>)))
        .or(parser(lines_found::<I>))
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
fn lines_hit<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let line_count = parser(integer_value::<I>)
        .map( | lines_hit | LCOVRecord::LinesHit(lines_hit) );

    between(string("LH:"), newline(), line_count).parse_state(input)
}

#[inline]
fn lines_found<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let line_found = parser(integer_value::<I>)
        .map( | line_found | LCOVRecord::LinesFound(line_found) );

    between(string("LF:"), newline(), line_found).parse_state(input)
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

#[cfg(test)]
mod tests {
    use super::*;
    use parser_combinators:: { parser, Parser };
    use record:: { LCOVRecord };

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
    fn end_of_record() {
        let result = parser(record).parse("end_of_record\n");
        assert_eq!(result.unwrap(), (LCOVRecord::EndOfRecord, ""));
    }
}
