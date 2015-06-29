use LcovRecord;
use RecordError;
use combinator:: { record };
use combinator;
use nom:: { IResult };
use lines::linereader:: { LineReader };
use std::io:: { Read, Error, ErrorKind, Result };
use std::error::Error as ErrorDescription;
use std::fs:: { File };
use std::str::{ from_utf8 };


pub fn record_from(input : &[u8]) -> Result<LcovRecord> {
    match combinator::record(input) {
        IResult::Done(_, record) => Ok(record),
        _ => Err(Error::new(ErrorKind::InvalidInput, "The record of file that can not be parsed."))
    }
}


pub trait LCOVParser {
    fn parse<R: Read>(&mut self, reader: R) {
        let mut lr = LineReader::new(reader);

        loop {
            let result = lr.read_line();
            let line = match result {
                Ok(b) if b.is_empty() => { break; },
                Ok(line) => {
                    match record_from(line) {
                        Ok(ref r) => self.complete(r),
                        Err(e) => {
                            let err = RecordError {
                                line_number: 0,
                                message: e.description().to_string(),
                                record: from_utf8(line).unwrap().to_string()
                            };
                            self.failed(&err)
                        }
                    }
                },
                Err(e) => { break; }
            };
        }
    }
    fn complete(&mut self, rc: &LcovRecord);
    fn failed(&mut self, error: &RecordError);
}

#[cfg(test)]
mod tests {
    use super::*;
    use LcovRecord;
    use RecordError;
    use std::fs::File;
    use std::io:: { Result };

    struct TestParser {
        records: Vec<LcovRecord>,
        record_errors: Vec<RecordError>
    }

    impl TestParser {
        fn new() -> Self {
            TestParser { records: vec!(), record_errors: vec!() }
        }
    }

    impl LCOVParser for TestParser {
        fn complete(&mut self, result: &LcovRecord) {
            self.records.push(result.clone());
        }
        fn failed(&mut self, error: &RecordError) {
            self.record_errors.push(error.clone());
        }
    }

    #[test]
    fn test_parse_from_file() {
        let mut f = File::open("./fixture/report.lcov").unwrap();
        let mut parser = TestParser::new();

        parser.parse(&f);

        assert_eq!(parser.records.len(), 1);
    }
}
