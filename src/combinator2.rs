//! The parser combinators for record.
//!
//! Supported record types are as follows.
//! Please see the following page for [the format](http://ltp.sourceforge.net/coverage/lcov/geninfo.1.php).
//!
//! * TN:<test name>
//! * SF:<absolute path to the source file>
//! * DA:<line number>,<execution count>[,<checksum>]
//! * end_of_record

use parser_combinators:: { many1, digit, string, satisfy, optional, token, value, sep_by, parser, Parser, ParserExt, ParseResult };
use parser_combinators::primitives:: { State, Stream };
use std::string:: { String };
use record:: { LCOVRecord };

#[derive(Debug, PartialEq)]
pub enum RecordResult {
    Record(LCOVRecord),
    RecordArray(Vec<LCOVRecord>)
}

#[inline]
pub fn record<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    parser(end_of_record::<I>)
        .or(parser(test_name::<I>))
        .or(parser(source_file::<I>))
        .or(parser(data::<I>))
        .parse_state(input)
}

#[inline]
pub fn records<I>(input: State<I>) -> ParseResult<RecordResult, I> where I: Stream<Item=char> {
    let array = sep_by(parser(record::<I>), token('\n'));

    parser(record::<I>).map(RecordResult::Record)
        .or(array.map(RecordResult::RecordArray))
        .parse_state(input)
}

#[inline]
fn test_name<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let test_name = parser(string_value::<I>).map( | s: String | LCOVRecord::TestName(s) );
    string("TN:").with(test_name).parse_state(input)
}

#[inline]
fn source_file<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let source_file =  parser(string_value::<I>).map( | s: String | LCOVRecord::SourceFile(s) );
    string("SF:").with(source_file).parse_state(input)
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
    string("DA:").with(record).parse_state(input)
}

#[inline]
fn end_of_record<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    string("end_of_record").with(value(LCOVRecord::EndOfRecord)).parse_state(input)
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
        let result = parser(record).parse("TN:test_name");
        assert_eq!(result.unwrap(), (LCOVRecord::TestName("test_name".to_string()), ""));
    }

    #[test]
    fn source_file() {
        let result = parser(record).parse("SF:/path/to/source.rs");
        assert_eq!(result.unwrap(), (LCOVRecord::SourceFile("/path/to/source.rs".to_string()), ""));
    }

    #[test]
    fn data() {
        let result = parser(record).parse("DA:1,2");
        assert_eq!(result.unwrap(), (LCOVRecord::Data(1, 2, None), ""));
    }

    #[test]
    fn data_with_checksum() {
        let result = parser(record).parse("DA:1,2,3sdfjiji56");
        assert_eq!(result.unwrap(), (LCOVRecord::Data(1, 2, Some("3sdfjiji56".to_string())), ""));
    }

    #[test]
    fn end_of_record() {
        let result = parser(record).parse("end_of_record");
        assert_eq!(result.unwrap(), (LCOVRecord::EndOfRecord, ""));
    }
}
