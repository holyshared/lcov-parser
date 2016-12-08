extern crate lcov_parser;

use lcov_parser:: { LCOVParser, LCOVRecord };

fn main() {
    let content = concat!(
        "TN:test_name\n",
        "SF:/path/to/source.rs\n",
        "DA:1,2\n",
        "DA:2,1\n",
        "DA:3,5\n",
        "end_of_record\n"
    );
    let mut parser = LCOVParser::new(content.as_bytes());

    loop {
        match parser.next().expect("parse the report") {
            None => { break; },
            Some(record) => match record {
                LCOVRecord::TestName(name) => println!("Test: {}", name.unwrap()),
                LCOVRecord::SourceFile(file_name) => println!("File: {}", file_name),
                LCOVRecord::Data(data) => println!("Line: {}, Executed: {}", data.line, data.count),
                LCOVRecord::EndOfRecord => println!("Finish"),
                _ => { continue; }
            }
        }
    }
}
