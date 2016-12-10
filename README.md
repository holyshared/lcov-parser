# lcov-parser

LCOV report parser for Rust.

[![Build Status](https://travis-ci.org/holyshared/lcov-parser.svg)](https://travis-ci.org/holyshared/lcov-parser)
[![Build Status](https://ci.appveyor.com/api/projects/status/q83stma2v57joiwy/branch/master?svg=true)](https://ci.appveyor.com/project/holyshared/lcov-parser/branch/master)
[![crates.io version](https://img.shields.io/crates/v/lcov-parser.svg)](https://crates.io/crates/lcov-parser)
[![License](https://img.shields.io/crates/l/lcov-parser.svg)](https://github.com/holyshared/lcov-parser/blob/master/LICENSE)

## Basic usage

Create a LCOVParser object, and then parse the data.

```rust
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
```

## Parsing the file

It can also be used to parse the report file.

```rust
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
```

## Parsing all

You can parse all using the parse method.

```rust
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
```

## Merge reports

You use merge_files to merge reports.  
You can save the merged report by specifying the file name.

```rust
extern crate lcov_parser;

use lcov_parser:: { merge_files };

fn main() {
    let trace_files = [
        "../../../tests/fixtures/fixture1.info",
        "../../../tests/fixtures/fixture2.info"
    ];
    let _ = match merge_files(&trace_files) {
        Ok(report) => report.save_as("/tmp/merged_report.info"),
        Err(err) => panic!(err)
    };
}
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.
