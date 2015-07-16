extern crate lcov_parser;

use lcov_parser:: { LCOVParser, ParsedResult };

fn main() {
    let records = "TN:testname\nSF:/path/to/source.rs\n".as_bytes();
    let mut parser = LCOVParser::new(records);

    loop {
        match parser.parse_next() {
            ParsedResult::Ok(record, _) => println!("{:?}", record),
            ParsedResult::Eof => { break; },
            ParsedResult::Err(error) => println!("{:?}", error)
        }
    }
}
