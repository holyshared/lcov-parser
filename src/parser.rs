use LcovRecord;
use combinator:: { parse_record };
use nom:: { IResult };
use lines::linereader:: { LineReader };
use std::io:: { Read };
use std::fs:: { File };
use std::str::{ from_utf8 };

pub struct RecordParser<R> {
    reader: LineReader<R>,
    records: Vec<LcovRecord>
}

impl<R: Read> RecordParser<R> {
    fn new(reader: R) -> Self {
        RecordParser {
            reader: LineReader::new(reader),
            records: vec!()
        }
    }
    fn parse(&mut self) {
        loop {
            match self.reader.read_line() {
                Ok(b) if b.is_empty() => { break; }
                Ok(line) => {
                    match parse_record(line) {
                        IResult::Done(_, output) => self.records.push(output),
                        IResult::Error(a) => println!("{:?}", a),
                        IResult::Incomplete(needed) => continue
                    }
                }
                Err(e) => {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use LcovRecord;
    use combinator:: { parse_record };
    use std::fs::File;

    #[test]
    fn test_parse_from_file() {
        let mut f = File::open("./fixture/report.lcov").unwrap();
        let mut parser = RecordParser::new(f);

        parser.parse();

        assert_eq!(parser.records.len(), 1);
    }
}
