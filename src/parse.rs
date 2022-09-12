use crate::Offset;

#[derive(Clone, Debug)]
pub struct OffsetParser;

impl clap::builder::TypedValueParser for OffsetParser {
    type Value = Offset;

    fn parse_ref(
        &self, _cmd: &clap::Command, _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr
    ) -> Result<Self::Value, clap::Error> {
        let str = value.to_str().unwrap_or_default();
        if let (Some(number), radix) = match str.get(0..2) {
            Some("0x") | Some("0X") => (str.get(2..), 16),
            Some("0b") | Some("0B") => (str.get(2..), 2),
            Some("0o") | Some("0O") => (str.get(2..), 8),
            Some(value)
                if value.starts_with('0') && !str.contains(['8', '9']) =>
            {
                (str.get(1..), 8)
            }
            _ => (Some(str), 10)
        } {
            return Ok(Offset(match usize::from_str_radix(number, radix) {
                Ok(value) => value,
                Err(err) => {
                    return Err(clap::Error::raw(
                        clap::ErrorKind::InvalidValue,
                        format!("Offset parsing failed ({}).", err)
                    ))
                }
            }))
        }
        unreachable!()
    }
}

#[cfg(test)]
#[path = "parse_test.rs"]
mod parse_test;
