extern crate lcov_parser;

use lcov_parser:: { ReportParser };
use std::fs:: { File };
use std::io:: { Read, ErrorKind };
use std::env:: { current_dir };

fn main() {
    let current_dir = current_dir().unwrap();
    let file = current_dir.join("../../../fixture/report.lcov");
    let read_file_path = file.clone();

    match File::open(file) {
        Ok(ref mut file) => {
            let mut buffer = String::new();
            let _ = file.read_to_string(&mut buffer);
            let records = ReportParser::new(buffer.as_str()).parse().unwrap();

            for record in records.iter() {
                println!("{:?}", record);
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
