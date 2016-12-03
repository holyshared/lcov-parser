// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use combine:: { try, token, between, parser, Parser, ParseResult, State, Stream };
use combine::char:: { string, newline };
use record:: { LCOVRecord, FunctionName, FunctionData };
use combinator::value:: { to_integer, to_string };

#[inline]
pub fn function_record<I>(input: State<I>) -> ParseResult<LCOVRecord, State<I>> where I: Stream<Item=char> {
    try(parser(function_name::<I>))
        .or(try(parser(function_data::<I>)))
        .or(try(parser(functions_found::<I>)))
        .or(parser(functions_hit::<I>))
        .parse_stream(input)
}

#[inline]
fn function_name<I>(input: State<I>) -> ParseResult<LCOVRecord, State<I>> where I: Stream<Item=char> {
    let line_number = parser(to_integer::<I>);
    let function_name = token(',').with( parser(to_string::<I>) );

    let record = (line_number, function_name).map( | t | {
        let (line_number, function_name) = t;
        let func = FunctionName { name: function_name, line: line_number };
        LCOVRecord::from(func)
    });
    between(string("FN:"), newline(), record).parse_stream(input)
}

#[inline]
fn function_data<I>(input: State<I>) -> ParseResult<LCOVRecord, State<I>> where I: Stream<Item=char> {
    let execution_count = parser(to_integer::<I>);
    let function_name = token(',')
        .with( parser(to_string::<I>) );

    let record = (execution_count, function_name).map( | t | {
        let (execution_count, function_name) = t;
        let func_data = FunctionData { name: function_name, count: execution_count };
        LCOVRecord::from(func_data)
    });
    between(string("FNDA:"), newline(), record).parse_stream(input)
}

#[inline]
fn functions_found<I>(input: State<I>) -> ParseResult<LCOVRecord, State<I>> where I: Stream<Item=char> {
    let functions_found = parser(to_integer::<I>)
        .map( | functions_found | LCOVRecord::FunctionsFound(functions_found) );
    between(string("FNF:"), newline(), functions_found).parse_stream(input)
}

#[inline]
fn functions_hit<I>(input: State<I>) -> ParseResult<LCOVRecord, State<I>> where I: Stream<Item=char> {
    let functions_hit = parser(to_integer::<I>)
        .map( | functions_hit | LCOVRecord::FunctionsHit(functions_hit) );
    between(string("FNH:"), newline(), functions_hit).parse_stream(input)
}
