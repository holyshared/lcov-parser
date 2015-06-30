//! The parser combinators for record.
//!
//! Supported record types are as follows.
//! Please see the following page for [the format](http://ltp.sourceforge.net/coverage/lcov/geninfo.1.php).
//!
//! * TN:<test name>
//! * SF:<absolute path to the source file>
//! * DA:<line number>,<execution count>[,<checksum>]
//! * end_of_record

use nom::{ line_ending };
use std::str::{ from_utf8, FromStr };
use record:: { LCOVRecord };

named!(test_name<&[u8], LCOVRecord>,
    chain!(
        tag!("TN:") ~
        test_name: map_res!(
            take_until!("\n"),
            from_utf8
        ) ~
        line_ending,
        || LCOVRecord::TestName(test_name.to_string())
    )
);

named!(source_file<&[u8], LCOVRecord>,
    chain!(
        tag!("SF:") ~
        file_name: map_res!(
            take_until!("\n"),
            from_utf8
        ) ~
        line_ending,
        || LCOVRecord::SourceFile(file_name.to_string())
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
            take_until_either!("\n,"),
            from_utf8
        ) ~
        checksum: opt!(
            chain!(
                tag!(",") ~
                checksum: map_res!(
                    take_until!("\n"),
                    from_utf8
                ),
                || { checksum.to_string() }
            )
        ) ~
        line_ending,
        || LCOVRecord::Data(
            FromStr::from_str(line_number).unwrap(),
            FromStr::from_str(executed_count).unwrap(),
            checksum
        )
    )
);

named!(end_of_record<&[u8], LCOVRecord>,
    chain!(
        tag!("end_of_record") ~
        line_ending,
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
        let expected = LCOVRecord::TestName("foo".to_string());
        let expected_remain_input = &b""[..];

        assert_eq!(result, IResult::Done(expected_remain_input, expected));
    }

    #[test]
    fn test_parse_source_file_record() {
        let result = record(b"SF:foo\n");
        let expected = LCOVRecord::SourceFile("foo".to_string());
        let expected_remain_input = &b""[..];

        assert_eq!(result, IResult::Done(expected_remain_input, expected));
    }

    #[test]
    fn test_parse_data_record() {
        let result = record(b"DA:2,10\n");
        let expected = LCOVRecord::Data(2, 10, None);
        let expected_remain_input = &b""[..];

        assert_eq!(result, IResult::Done(expected_remain_input, expected));
    }

    #[test]
    fn test_parse_data_record_with_checksum() {
        let result = record(b"DA:2,10,abcd\n");
        let expected = LCOVRecord::Data(2, 10, Some("abcd".to_string()));
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
