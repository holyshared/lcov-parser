use LcovRecord;
use combinator:: { record };
use combinator;
use nom:: { IResult };
use lines::linereader:: { LineReader };
use std::io:: { Read, Error, ErrorKind, Result };
use std::fs:: { File };
use std::str::{ from_utf8 };


pub fn record_from(input : &[u8]) -> Result<LcovRecord> {
    match combinator::record(input) {
        IResult::Done(_, record) => Ok(record),
        _ => Err(Error::new(ErrorKind::InvalidInput, "The record of file that can not be parsed."))
    }
}

pub struct RecordParser<R> {
    reader: LineReader<R>
}

pub trait RecordReceiver {
    fn receive(&mut self, result : &Result<LcovRecord>);
}

impl<R: Read> RecordParser<R> {
    fn new(reader: R) -> Self {
        RecordParser {
            reader: LineReader::new(reader)
        }
    }
    fn parse<P: RecordReceiver>(&mut self, receiver: &mut P) {
        loop {
            let result = self.reader.read_line();
            let line = match result {
                Ok(b) if b.is_empty() => { break; },
                Ok(line) => {
                    let r = record_from(line);
                    receiver.receive(&r);
                }
                Err(e) => { break; }
            };
        }
    }
}






#[cfg(test)]
mod tests {
    use super::*;
    use LcovRecord;
    use std::fs::File;
    use std::io:: { Result };

    struct TestReceiver {
        record_count: u32
    }

    impl RecordReceiver for TestReceiver {
        fn receive(&mut self, result: &Result<LcovRecord>) {
            self.record_count = self.record_count + 1;
        }
    }

    #[test]
    fn test_parse_from_file() {
        let mut r = TestReceiver { record_count: 0 };
        let mut f = File::open("./fixture/report.lcov").unwrap();
        let mut parser = RecordParser::new(f);

        parser.parse(&mut r);

        assert_eq!(r.record_count, 1);
    }
}
