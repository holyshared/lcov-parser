lcov-parser
================================

LCOV report parser for Rust.

[![Build Status](https://travis-ci.org/holyshared/lcov-parser.svg)](https://travis-ci.org/holyshared/lcov-parser)
[![crates.io version](https://img.shields.io/crates/v/lcov-parser.svg)](https://crates.io/crates/lcov-parser)
[![License](https://img.shields.io/github/license/mashape/apistatus.svg)](https://github.com/holyshared/lcov-parser/blob/master/LICENSE)

Basic usage
--------------------------------

If you simply want to parse, use the **each_records**.  
You can run the parsing process on a record-by-record basis in **each_records**.

```rust
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
```

LCOVParser
--------------------------------

When you use the **LCOVParser**, it will be able to control the processing of parse.

```rust
let records = "TN:testname\nSF:/path/to/source.rs\n".as_bytes();
let mut parser = LCOVParser::new(records);

loop {
	match parser.parse_next() {
		ParsedResult::Ok(record, _) => println!("{:?}", record),
		ParsedResult::Eof => { break; },
		ParsedResult::Err(error) => println!("{:?}", error)
	}
}
```

It can also be used to parse the report file.


```rust
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
	Err(error) => panic!("{:?}", error)
}
```
