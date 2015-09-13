use combine:: { string, try, token, between, newline, parser, Parser, ParserExt, ParseResult };
use combine::primitives:: { State, Stream };
use record:: { LCOVRecord };
use combinator::value:: { to_integer, to_string };

#[inline]
pub fn function_record<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    try(parser(function_name::<I>))
        .or(try(parser(function_data::<I>)))
        .or(try(parser(functions_found::<I>)))
        .or(parser(functions_hit::<I>))
        .parse_state(input)
}

#[inline]
fn function_name<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let line_number = parser(to_integer::<I>);
    let function_name = token(',').with( parser(to_string::<I>) );

    let record = (line_number, function_name).map( | t | {
        let (line_number, function_name) = t;
        LCOVRecord::FunctionName(line_number, function_name)
    });
    between(string("FN:"), newline(), record).parse_state(input)
}

#[inline]
fn function_data<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let execution_count = parser(to_integer::<I>);
    let function_name = token(',')
        .with( parser(to_string::<I>) );

    let record = (execution_count, function_name).map( | t | {
        let (execution_count, function_name) = t;
        LCOVRecord::FunctionData(execution_count, function_name)
    });
    between(string("FNDA:"), newline(), record).parse_state(input)
}

#[inline]
fn functions_found<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let functions_found = parser(to_integer::<I>)
        .map( | functions_found | LCOVRecord::FunctionsFound(functions_found) );
    between(string("FNF:"), newline(), functions_found).parse_state(input)
}

#[inline]
fn functions_hit<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let functions_hit = parser(to_integer::<I>)
        .map( | functions_hit | LCOVRecord::FunctionsHit(functions_hit) );
    between(string("FNH:"), newline(), functions_hit).parse_state(input)
}
