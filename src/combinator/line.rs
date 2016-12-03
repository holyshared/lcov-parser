// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use combine:: { string, try, between, newline, parser, Parser, ParserExt, ParseResult };
use combine::primitives:: { State, Stream };
use record:: { LCOVRecord };
use combinator::value:: { to_integer };

#[inline]
pub fn lines_record<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    try(parser(lines_hit::<I>))
        .or(parser(lines_found::<I>))
        .parse_stream(input)
}

#[inline]
fn lines_hit<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let line_count = parser(to_integer::<I>)
        .map( | lines_hit | LCOVRecord::LinesHit(lines_hit) );

    between(string("LH:"), newline(), line_count).parse_stream(input)
}

#[inline]
fn lines_found<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let line_found = parser(to_integer::<I>)
        .map( | line_found | LCOVRecord::LinesFound(line_found) );

    between(string("LF:"), newline(), line_found).parse_stream(input)
}
