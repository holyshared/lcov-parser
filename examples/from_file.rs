extern crate lcov_parser;

use lcov_parser:: { LCOVParser, LCOVRecord, FromFile };

fn main() {
    let mut parser = LCOVParser::from_file("../../../fixture/report.lcov").unwrap();

    loop {
        match parser.next().expect("parse the report") {
            None => { break; },
            Some(record) => match record {
                LCOVRecord::SourceFile(file_name) => println!("File: {}", file_name),
                LCOVRecord::EndOfRecord => println!("Finish"),
                _ => { continue; }
            }
        }
    }
}
