extern crate lcov_parser;

use std::fs:: { File };
use std::io:: { Result };
use std::env:: { current_dir };
use lcov_parser:: { LCOVParser, LCOVRecord, ParsedResult };

#[derive(Clone)]
pub struct LineResult {
    executed: i32,
    unused: i32
}

#[derive(Clone)]
pub struct FileResult {
    name: String,
    lines: LineResult
}

#[derive(Clone)]
pub struct CoverageResult {
    files: Vec<FileResult>
}

fn open_fixture(path: &str) -> Result<File> {
    let current_dir = current_dir().unwrap();
    let file = current_dir.join(path);
    let fixture_file = try!(File::open(file));
    Ok(fixture_file)
}

#[test]
fn without_checksum() {
    let report = open_fixture("fixture/report.lcov").unwrap();
    let mut parser = LCOVParser::new(report);

    let mut result = CoverageResult { files: vec!() };
    let mut file_result = FileResult { name: "".to_string(), lines: LineResult { executed: 0, unused: 0 } };
    let mut line_result = LineResult { executed: 0, unused: 0 };

    loop {
        match parser.parse_next() {
            ParsedResult::Ok(record, _) => {
                match record {
                    LCOVRecord::SourceFile(name) => {
                        line_result = LineResult { executed: 0, unused: 0 };
                        file_result = FileResult { name: name, lines: LineResult { executed: 0, unused: 0 } };
                    },
                    LCOVRecord::Data(_, executed_count, _) => {
                        if executed_count >= 1 {
                            line_result.executed = line_result.executed + 1;
                        } else {
                            line_result.unused = line_result.unused + 1;
                        }
                    },
                    LCOVRecord::EndOfRecord => {
                        file_result.lines = line_result.clone();
                        result.files.push( file_result.clone() );
                    }
                    _ => { continue; }
                }
            },
            ParsedResult::Eof => { break; },
            ParsedResult::Err(error) => println!("{:?}", error)
        }
    }

    assert_eq!(parser.current_record_count(), 13);
    assert_eq!(result.files.len(), 2);

    let f1 = result.files.get(0).unwrap();
    assert_eq!(f1.lines.executed, 4);
    assert_eq!(f1.lines.unused, 0);

    let f2 = result.files.get(1).unwrap();
    assert_eq!(f2.lines.executed, 0);
    assert_eq!(f2.lines.unused, 4);
}
