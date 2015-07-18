extern crate lcov_parser;

use lcov_parser:: { LCOVParser, ParsedResult };
use std::fs:: { File };
use std::io:: { ErrorKind };
use std::env:: { current_dir };

fn main() {
    let current_dir = current_dir().unwrap();
    let file = current_dir.join("../../../fixture/report.lcov");
    let read_file_path = file.clone();

    match File::open(file) {
        Ok(file) => {
            let mut parser = LCOVParser::new(file);

            loop {
                match parser.parse_next() {
                    ParsedResult::Ok(record, _) => println!("{:?}", record),
                    ParsedResult::Eof => { break; },
                    ParsedResult::Err(error) => println!("{:?}", error)
                }
            }
        },
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => panic!("File not found: {:?}", read_file_path),
                ErrorKind::PermissionDenied => panic!("Permission denied"),
                _ => panic!("{:?}", error)
            }
        }
    }
}
