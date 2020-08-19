//
use super::Error;
use std::fmt;
use std::ops::Range;
use std::str::FromStr;

// Hold a mode
#[derive(Debug)]
pub struct Mode(usize);

const BITS: &[&str] = &[
    "---",
    "--x",
    "-w-",
    "-wx",
    "r--",
    "r-x",
    "rw-",
    "rwx",
];

// Mask for discovering permissions
const BITS_MASK: usize = 7;

// Masks for detecting extra bits.
// Sticky bit
const S_ISTXT: usize = 0o1000;
// Set GID
const S_ISGID: usize = 0o2000;
// Set UID
const S_ISUID: usize = 0o4000;

const S_IXUSR: usize = 0o0100;
const S_IXGRP: usize = 0o0010;
const S_IXOTH: usize = 0o0001;

// Shift amounts for calculating permissions.
const SHIFT_OTH: usize = 0;
const SHIFT_GRP: usize = 3;
const SHIFT_USR: usize = 6;

// Max size of an octal digit.
const OCTAL_MAX: usize = 7;

// Number of bits to shift octal binary by when converting string to number
const OCTAL_DIGIT_SIZE: usize = 3;

// Replacement range for permission bits
const BITS_RANGE: Range<usize> = 2..3;

impl Mode {
    pub fn new(mode: usize) -> Self {
        Self(mode)
    }

    fn permission(&self, shift: usize) -> String {
        let bits  = self.0 >> shift;
        let index = bits & BITS_MASK;

        BITS[index].to_owned()
    }

    fn group(&self) -> String {
        let mut s = self.permission(SHIFT_GRP);

        if self.is_sgid() {
            let replacement = if self.is_exec(S_IXGRP) {
                "s"
            }
            else {
                "S"
            };

            s.replace_range(BITS_RANGE, replacement);
        }

        s
    }

    fn other(&self) -> String {
        let mut s = self.permission(SHIFT_OTH);

        if self.is_sticky() {
            let replacement = if self.is_exec(S_IXOTH) {
                "t"
            }
            else {
                "T"
            };

            s.replace_range(BITS_RANGE, replacement);
        }

        s
    }

    fn user(&self) -> String {
        let mut s = self.permission(SHIFT_USR);

        if self.is_suid() {
            let replacement = if self.is_exec(S_IXUSR) {
                "s"
            }
            else {
                "S"
            };

            s.replace_range(BITS_RANGE, replacement);
        }

        s
    }

    fn is_sgid(&self) -> bool {
        self.0 & S_ISGID > 0
    }

    fn is_suid(&self) -> bool {
        self.0 & S_ISUID > 0
    }

    fn is_sticky(&self) -> bool {
        self.0 & S_ISTXT > 0
    }

    fn is_exec(&self, mask: usize) -> bool {
        self.0 & mask > 0
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let user  = self.user();
        let group = self.group();
        let other = self.other();

        write!(f, "{}{}{}", user, group, other)
    }
}

impl FromStr for Mode {
    type Err = Error;

    // Takes a string and converts it to octal
    // Can fail if numbers are over 7, or we can't parse the input to a number
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut output: usize = 0;

        for c in input.trim().chars() {
            let i: usize = c.to_string().parse()?;

            if i > OCTAL_MAX {
                let err = Error::DigitTooLarge(i);

                return Err(err);
            }

            output = (output << OCTAL_DIGIT_SIZE) | i;
        }

        Ok(Self::new(output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_from_octal() {
        let tests = vec![
            (0o000,  "---------"),
            (0o644,  "rw-r--r--"),
            (0o755,  "rwxr-xr-x"),
            (0o0000, "---------"),
            (0o0644, "rw-r--r--"),
            (0o0755, "rwxr-xr-x"),
            (0o1712, "rwx--x-wT"),
            (0o1000, "--------T"),
            (0o1001, "--------t"),
            (0o2000, "-----S---"),
            (0o2010, "-----s---"),
            (0o4000, "--S------"),
            (0o4100, "--s------"),
            (0o7666, "rwSrwSrwT"),
            (0o7777, "rwsrwsrwt"),
        ];

        for test in tests {
            let input    = test.0;
            let expected = test.1;
            let mode     = Mode::new(input).to_string();

            assert_eq!(expected, mode)
        }
    }

    #[test]
    fn test_display_from_string() {
        let tests = vec![
            ("000",  "---------"),
            ("644",  "rw-r--r--"),
            ("755",  "rwxr-xr-x"),
            ("0000", "---------"),
            ("0644", "rw-r--r--"),
            ("0755", "rwxr-xr-x"),
            ("1712", "rwx--x-wT"),
            ("1000", "--------T"),
            ("2000", "-----S---"),
            ("4000", "--S------"),
            ("1001", "--------t"),
            ("2010", "-----s---"),
            ("4100", "--s------"),
            ("7666", "rwSrwSrwT"),
            ("7777", "rwsrwsrwt"),
        ];

        for test in tests {
            let input    = test.0;
            let expected = test.1;
            let mode     = Mode::from_str(input).unwrap().to_string();

            assert_eq!(expected, mode)
        }
    }
}
