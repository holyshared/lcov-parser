extern crate lcov_parser;

use lcov_parser:: { LCOVParser, LCOVRecord, FromFile };

fn main() {
    let records = {
        let mut parser = LCOVParser::from_file("../../../fixture/report.lcov").unwrap();
        parser.parse().expect("parse the report")
    };

    for record in records.iter() {
        match record {
            &LCOVRecord::SourceFile(ref file_name) => println!("File: {}", file_name),
            &LCOVRecord::EndOfRecord => println!("Finish"),
            _ => { continue; }
        }
    }
}
