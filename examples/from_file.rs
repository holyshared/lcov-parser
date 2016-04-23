extern crate lcov_parser;

use lcov_parser:: { LCOVParser };

fn main() {
    let parser = LCOVParser::from_file("../../../fixture/report.lcov").unwrap();
    let records = parser.parse().unwrap();

    for record in records.iter() {
        println!("{:?}", record);
    }
}
