use parser_combinators:: { many1, digit, satisfy, Parser, ParserExt, ParseResult };
use parser_combinators::primitives:: { State, Stream };

pub fn to_integer<I>(input: State<I>) -> ParseResult<u32, I> where I: Stream<Item=char> {
    many1( digit() )
        .map( |s: String| s.parse::<u32>().unwrap() )
        .parse_state(input)
}

pub fn to_string<I>(input: State<I>) -> ParseResult<String, I> where I: Stream<Item=char> {
    many1( satisfy( | c: char | c != '\n' ) )
        .map( | s: String | s )
        .parse_state(input)
}
