extern crate lcov_parser;

use lcov_parser:: { each_records };
use std::fs:: { File };
use std::io:: { Read };
use std::path:: { PathBuf };
use std::env:: { args, current_dir };

fn main() {
    let args = args();

    if args.len() <= 1 {
        panic!("Please specify the file.");
    };

    let relative_path = args.last().unwrap();
    let report = report_path(&relative_path[..]);
    let report_path = report.clone();
    let mut result = File::open(report);

    match result.as_mut() {
        Ok(&mut ref mut file) => print_records(file),
        Err(_) => panic!("Could not open file {:?}", report_path)
    }
}

fn report_path(relative_path: &str) -> PathBuf {
    let cwd = current_dir().unwrap();
    return cwd.join(relative_path);
}

fn print_records(file: &mut File) {
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(_) => {
            println!("Start the parse report\n");

            each_records(buffer.as_bytes(), | record | {
                println!("{:?}", record);
            });

            println!("\nFinish the parse report");
        },
        Err(error) => panic!("{:?}", error)
    }
}
