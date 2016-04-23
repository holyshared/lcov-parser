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

    let records = LCOVParser::new(content).parse().unwrap();

    for record in records.into_iter() {
        match record {
            LCOVRecord::TestName(name) => println!("Test: {}", name),
            LCOVRecord::SourceFile(file_name) => println!("File: {}", file_name),
            LCOVRecord::Data(line_number, execution_count, _) => println!("Line: {}, Executed: {}", line_number, execution_count),
            LCOVRecord::EndOfRecord => println!("Finish"),
            _ => { continue; }
        }
    }
}
```

## Parsing the file

It can also be used to parse the report file.

```rust
let parser = LCOVParser::from_file("/path/to/report.lcov").unwrap();
let records = parser.parse().unwrap();

for record in records.iter() {
    match record {
        &LCOVRecord::SourceFile(ref name) => println!("start file: {}", name),
        &LCOVRecord::EndOfRecord => println!("end file"),
        _ => { continue; }
    }
}
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.
