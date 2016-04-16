extern crate lcov_parser;

use lcov_parser:: { LCOVParser };

fn main() {
    let input = "TN:testname\nSF:/path/to/source.rs\n";
    let records = LCOVParser::new(input).parse().unwrap();

    for record in records.iter() {
        println!("{:?}", record);
    }
}
