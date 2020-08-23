//
use super::Error;
use std::ops::RangeInclusive;
use std::str::FromStr;

const VALID_FILE_MODE: &[char] = &['-', 'r', 'w', 'x', 's', 'S', 't', 'T'];
const VALID_OCTAL: RangeInclusive<char> = RangeInclusive::new('0', '7');

#[derive(Debug, PartialEq)]
pub enum ModeType {
    FileMode,
    OctalNumeric,
}

impl FromStr for ModeType {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.chars().all(|c| VALID_OCTAL.contains(&c)) {
            let mt = Self::OctalNumeric;

            return Ok(mt);
        }

        if input.chars().all(|c| VALID_FILE_MODE.contains(&c)) {
            let mt = Self::FileMode;

            return Ok(mt);
        }

        Err(Error::InvalidModeString)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_ok() {
        let tests = vec![
            ("777",       Ok(ModeType::OctalNumeric)),
            ("4755",      Ok(ModeType::OctalNumeric)),
            ("4758",      Err(Error::InvalidModeString)),
            ("rwxrwxrwx", Ok(ModeType::FileMode)),
            ("rwsrwsrwt", Ok(ModeType::FileMode)),
            ("rwSrwSrwT", Ok(ModeType::FileMode)),
            ("rwSrwSrwz", Err(Error::InvalidModeString)),
        ];

        for test in tests {
            let input     = test.0;
            let expected  = test.1;
            let mode_type = ModeType::from_str(input);

            assert_eq!(expected, mode_type)
        }
    }
}
