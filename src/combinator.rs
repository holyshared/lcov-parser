use nom::{ line_ending, eof };
use std::str::{ from_utf8, FromStr };
use record:: { LCOVRecord };

named!(test_name<&[u8], LCOVRecord>,
    chain!(
        tag!("TN:") ~
        test_name: map_res!(
            take_until!("\n"),
            from_utf8
        ) ~
        line_ending ~
        eof,
        || { LCOVRecord::TestName { name: test_name.to_string() } }
    )
);

named!(source_file<&[u8], LCOVRecord>,
    chain!(
        tag!("SF:") ~
        file_name: map_res!(
            take_until!("\n"),
            from_utf8
        ) ~
        line_ending ~
        eof,
        || LCOVRecord::SourceFile { file_name: file_name.to_string() }
    )
);

named!(data<&[u8], LCOVRecord>,
    chain!(
        tag!("DA:") ~
        line_number: map_res!(
            take_until!(","),
            from_utf8
        ) ~
        tag!(",") ~
        executed_count: map_res!(
            take_until!("\n"),
            from_utf8
        ) ~
        line_ending ~
        eof,
        || LCOVRecord::Data {
            line_number: FromStr::from_str(line_number).unwrap(),
            executed_count: FromStr::from_str(executed_count).unwrap()
        }
    )
);

named!(end_of_record<&[u8], LCOVRecord>,
    chain!(
        tag!("end_of_record") ~
        line_ending ~
        eof,
        || LCOVRecord::EndOfRecord
    )
);

named!(pub record<&[u8], LCOVRecord>,
    alt!(test_name | source_file | data | end_of_record)
);

named!(pub records<&[u8], Vec<LCOVRecord> >, many1!(record));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ IResult };
    use record:: { LCOVRecord };

    #[test]
    fn test_parse_tn_record() {
        let result = record(b"TN:foo\n");
        let expected = LCOVRecord::TestName { name: "foo".to_string() };
        let expected_remain_input = &b""[..];

        assert_eq!(result, IResult::Done(expected_remain_input, expected));
    }

    #[test]
    fn test_parse_source_file_record() {
        let result = record(b"SF:foo\n");
        let expected = LCOVRecord::SourceFile { file_name: "foo".to_string() };
        let expected_remain_input = &b""[..];

        assert_eq!(result, IResult::Done(expected_remain_input, expected));
   }

   #[test]
   fn test_parse_data_record() {
       let result = record(b"DA:2,10\n");
       let expected = LCOVRecord::Data { line_number: 2, executed_count: 10 };
       let expected_remain_input = &b""[..];

       assert_eq!(result, IResult::Done(expected_remain_input, expected));
   }

    #[test]
    fn test_parse_end_of_record() {
        let result = record(b"end_of_record\n");
        let expected_remain_input = &b""[..];

        assert_eq!(result, IResult::Done(expected_remain_input, LCOVRecord::EndOfRecord));
    }
}
