mod parse_ref_test {
    use std::ffi::OsStr;

    use clap::{builder::TypedValueParser, ErrorKind};

    use crate::{parse::OffsetParser, Offset};

    #[test]
    fn it_returns_an_error_when_parsing_has_failed() {
        let command = clap::Command::new("test");
        let parser = OffsetParser {};
        let result = parser.parse_ref(&command, None, OsStr::new("ZZZ"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn it_returns_parsed_offset() -> Result<(), clap::Error> {
        let command = clap::Command::new("test");
        let parser = OffsetParser {};
        for (input, expected_offset) in [
            (OsStr::new("0xBEEF"), Offset(48879)),
            (OsStr::new("0b0101"), Offset(5)),
            (OsStr::new("0o103"), Offset(67)),
            (OsStr::new("066"), Offset(54)),
            (OsStr::new("123"), Offset(123))
        ] {
            assert_eq!(
                parser.parse_ref(&command, None, input)?,
                expected_offset
            );
        }

        Ok(())
    }
}
