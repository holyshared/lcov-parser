extern crate lcov_parser;

use std::fs:: { File };
use lcov_parser:: { LCOVParser };

fn main() {
    let s = File::open("../../../fixture/report.lcov").unwrap();

    let mut records = vec![];
    let mut parser = LCOVParser::new(s);

    loop {
        let result = parser.next().unwrap();
        if result.is_none() {
            break;
        }
        let record = result.unwrap();
        records.push(record)
    }
    println!("{:?}", records);
    println!("{:?}", records.len());
}
