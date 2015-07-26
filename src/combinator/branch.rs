use parser_combinators:: { many1, digit, string, satisfy, optional, token, value, try, between, newline, parser, Parser, ParserExt, ParseResult };
use parser_combinators::primitives:: { State, Stream };
use record:: { LCOVRecord, Token };
use combinator::value:: { integer_value, string_value };

pub fn branch_data<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
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

pub fn branches_found<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let branches_found = parser(integer_value::<I>)
        .map( | branches_found | LCOVRecord::BranchesFound(branches_found) );

    between(string("BRF:"), newline(), branches_found).parse_state(input)
}

pub fn branches_hit<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let branches_hit = parser(integer_value::<I>)
        .map( | branches_hit | LCOVRecord::BranchesHit(branches_hit) );

    between(string("BRH:"), newline(), branches_hit).parse_state(input)
}
