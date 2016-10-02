// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use combine:: { string, optional, token, value, between, newline, parser, Parser, ParserExt, ParseResult };
use combine::primitives:: { State, Stream };
use std::string:: { String };
use record:: { LCOVRecord, LineData  };
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
    let test_name = optional(parser(to_string::<I>))
        .map(| s: Option<String> | LCOVRecord::TestName(s));
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
        let line = LineData {
            line: line_number,
            count: execution_count,
            checksum: checksum
        };
        LCOVRecord::from(line)
    });
    between(string("DA:"), newline(), record).parse_state(input)
}

#[inline]
fn end_of_record<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    between(string("end_of_record"), newline(), value(LCOVRecord::EndOfRecord)).parse_state(input)
}
