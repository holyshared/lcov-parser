extern crate lcov_parser;

use lcov_parser:: { each_records, LCOVRecord };

fn main() {
    let content = concat!(
        "TN:test_name\n",
        "SF:/path/to/source.rs\n",
        "DA:1,2\n",
        "DA:2,1\n",
        "DA:3,5\n",
        "end_of_record\n"
    );

    each_records(content.as_bytes(), | record | {
        match record {
            LCOVRecord::TestName(name) => println!("Test: {}", name),
            LCOVRecord::SourceFile(file_name) => println!("File: {}", file_name),
            LCOVRecord::Data(line_number, execution_count, _) => println!("Line: {}, Executed: {}", line_number, execution_count),
            LCOVRecord::EndOfRecord => println!("Finish")
        }
    });
}
