// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use combine:: { many1, digit, satisfy, Parser, ParserExt, ParseResult };
use combine::primitives:: { State, Stream };

pub fn to_integer<I>(input: State<I>) -> ParseResult<u32, I> where I: Stream<Item=char> {
    many1( digit() )
        .map( |s: String| s.parse::<u32>().unwrap() )
        .parse_stream(input)
}

pub fn to_string<I>(input: State<I>) -> ParseResult<String, I> where I: Stream<Item=char> {
    many1( satisfy( | c: char | c != '\n' ) )
        .map( | s: String | s )
        .parse_stream(input)
}
