//
use super::Error;
use std::ops::RangeInclusive;
use std::str::FromStr;

// Possible characters in a file mode.
const VALID_FILE_MODE: &[char] = &['-', 'r', 'w', 'x', 's', 'S', 't', 'T'];

// Possible characters in an octal string
const VALID_OCTAL: RangeInclusive<char> = RangeInclusive::new('0', '7');

// Characters allowed in positions that can't contain special characters
const NORMAL_CHARS: &[char] = &['-', 'r', 'w'];

// Characters allowed in the usr or grp exec fields
const USRGRP_EXEC_CHARS: &[char] = &['-', 'x', 's', 'S'];

// Characters allowed in the oth exec field
const OTH_EXEC_CHARS: &[char] = &['-', 'x', 't', 'T'];

#[derive(Debug, PartialEq)]
pub enum ModeType {
    // Represents a string style mode, eg. rwxr-xr-x
    FileMode,
    // Represents an octal mode, eg. 0755
    OctalNumeric,
}

impl FromStr for ModeType {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Valid input lengths
        // 3: 755
        // 4: 1755
        // 9: rwxrwxrwx
        if ![3, 4, 9].contains(&input.len()) {
            return Err(Error::InvalidModeString);
        }

        // Check for input composed entirely of valid octal characters
        if input.chars().all(|c| VALID_OCTAL.contains(&c)) {
            return Ok(Self::OctalNumeric);
        }

        // Check for input composed entirely of valid file mode characters
        if input.chars().all(|c| VALID_FILE_MODE.contains(&c)) {
            for (i, c) in input.chars().enumerate() {
                match i {
                    // Indexes that cannot contain suid, sgid, or sticky bits
                    0 | 1 | 3 | 4 | 6 | 7 => {
                        if !NORMAL_CHARS.contains(&c) {
                            return Err(Error::InvalidModeString);
                        }
                    },
                    // Indexes that could be suid or sgid
                    2 | 5 => {
                        if !USRGRP_EXEC_CHARS.contains(&c) {
                            return Err(Error::InvalidModeString);
                        }
                    },
                    8 => {
                        if !OTH_EXEC_CHARS.contains(&c) {
                            return Err(Error::InvalidModeString);
                        }
                    },
                    _ => unreachable!(),
                }
            }

            return Ok(Self::FileMode);
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
