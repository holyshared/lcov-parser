use parser_combinators:: { string, optional, token, value, between, newline, parser, Parser, ParserExt, ParseResult };
use parser_combinators::primitives:: { State, Stream };
use std::string:: { String };
use record:: { LCOVRecord  };
use combinator::value:: { to_integer, to_string };

#[inline]
pub fn general_record<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    parser(end_of_record::<I>)
        .or(parser(test_name::<I>))
        .or(parser(source_file::<I>))
        .or(parser(data::<I>))
        .parse_state(input)
}

#[inline]
fn test_name<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let test_name = parser(to_string::<I>).map( | s: String | LCOVRecord::TestName(s) );
    between(string("TN:"), newline(), test_name).parse_state(input)
}

#[inline]
fn source_file<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let source_file =  parser(to_string::<I>).map( | s: String | LCOVRecord::SourceFile(s) );
    between(string("SF:"), newline(), source_file).parse_state(input)
}

#[inline]
fn data<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let line_number = parser(to_integer::<I>);
    let execution_count = token(',').with( parser(to_integer::<I>) );
    let checksum = optional( token(',').with( parser(to_string::<I>) ) );
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
