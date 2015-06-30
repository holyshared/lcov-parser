extern crate lcov_parser;

use lcov_parser:: { each_records };
use std::fs:: { File };
use std::io:: { Read };
use std::path:: { PathBuf };
use std::env:: { current_dir };

fn main() {
    let report = report_path();
    let report_path = report.clone();
    let mut result = File::open(report);

    match result.as_mut() {
        Ok(&mut ref mut file) => print_records(file),
        Err(_) => panic!("Could not open file {:?}", report_path)
    }
}

fn report_path() -> PathBuf {
    let cwd = current_dir().unwrap();
    return cwd.join("../../../fixture/report.lcov");
}

fn print_records(file: &mut File) {
    let mut buffer = String::new();

    match file.read_to_string(&mut buffer) {
        Ok(_) => {
            each_records(buffer.as_bytes(), | record | {
                println!("{:?}", record);
            })
        },
        Err(error) => panic!("{:?}", error)
    }
}
