// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use combine:: { string, token, value, try, between, newline, parser, Parser, ParserExt, ParseResult };
use combine::primitives:: { State, Stream };
use record:: { LCOVRecord };
use combinator::value:: { to_integer };

#[inline]
pub fn branch_record<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    try(parser(branch_data::<I>))
        .or(try(parser(branches_found::<I>)))
        .or(parser(branches_hit::<I>))
        .parse_state(input)
}

#[inline]
fn branch_data<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let line_number = parser(to_integer::<I>);
    let block_number = token(',').with( parser(to_integer::<I>) );
    let branch_number = token(',').with( parser(to_integer::<I>) );

    let called = parser(to_integer::<I>);
    let not_called = token('-').with( value(0) );

    let branch_execution_count = try(not_called).or(called);

    let taken = token(',').with(branch_execution_count);

    let record = (line_number, block_number, branch_number, taken).map( | t | {
        let (line_number, block_number, branch_number, taken) = t;
        LCOVRecord::BranchData(line_number, block_number, branch_number, taken)
    });
    between(string("BRDA:"), newline(), record).parse_state(input)
}

#[inline]
fn branches_found<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let branches_found = parser(to_integer::<I>)
        .map( | branches_found | LCOVRecord::BranchesFound(branches_found) );

    between(string("BRF:"), newline(), branches_found).parse_state(input)
}

#[inline]
fn branches_hit<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let branches_hit = parser(to_integer::<I>)
        .map( | branches_hit | LCOVRecord::BranchesHit(branches_hit) );

    between(string("BRH:"), newline(), branches_hit).parse_state(input)
}
