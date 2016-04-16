extern crate lcov_parser;

use lcov_parser:: { ReportParser };

fn main() {
    let input = "TN:testname\nSF:/path/to/source.rs\n";
    let records = ReportParser::new(input).parse().unwrap();

    for record in records.iter() {
        println!("{:?}", record);
    }
}
