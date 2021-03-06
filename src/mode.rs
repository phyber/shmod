//
use super::Error;
use super::ModeType;
use std::fmt;
use std::ops::Range;
use std::str::FromStr;

// Hold a mode
#[derive(Debug)]
pub struct Mode(usize);

const BITS: &[&str] = &[
    "---", // 0b000 // 0
    "--x", // 0b001 // 1
    "-w-", // 0b010 // 2
    "-wx", // 0b011 // 3
    "r--", // 0b100 // 4
    "r-x", // 0b101 // 5
    "rw-", // 0b110 // 6
    "rwx", // 0b111 // 7
];

// Mask for discovering permissions
const BITS_MASK: usize = 0b0111;

// Masks for detecting extra bits.
// Sticky bit
const S_ISTXT: usize = 0o1000;
// Set GID
const S_ISGID: usize = 0o2000;
// Set UID
const S_ISUID: usize = 0o4000;

// Executable bits for user, group, other.
const S_IXUSR: usize = 0o0100;
const S_IXGRP: usize = 0o0010;
const S_IXOTH: usize = 0o0001;

// Shift amounts for calculating string permissions.
const SHIFT_OTH: usize = 0;
const SHIFT_GRP: usize = 3;
const SHIFT_USR: usize = 6;

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

    fn set_bits(&mut self, bits: usize) {
        self.0 |= bits
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{octal:03o}: {user}{group}{other}",
            octal = self.0,
            user = self.user(),
            group = self.group(),
            other = self.other(),
        )
    }
}

impl FromStr for Mode {
    type Err = Error;

    // Takes a string and converts it to octal
    // Can fail if numbers are over 7, or we can't parse the input to a number
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Trims leading and trailing whitespace from the input before
        // processing it.
        let input    = input.trim();
        let mut mode = Self::new(0);

        // Work out which type of mode string we're dealing with and handle it
        match ModeType::from_str(input)? {
            ModeType::OctalNumeric => {
                // Characters were checked for validity during ModeType parsing
                // so we should always parse successfully here and no digits
                // should be > 7.
                // String is processed in reverse to ensure bits are shifted
                // correctly.
                for (i, c) in input.chars().rev().enumerate() {
                    let digit: usize = c.to_string().parse()?;

                    // Octal is 3 bits, so we shift by 3 for each digit.
                    let bits = digit << (3 * i);
                    mode.set_bits(bits);
                }
            },
            ModeType::FileMode => {
                let mut index = 8;

                for (i, c) in input.chars().enumerate() {
                    match c {
                        'r' | 'w' | 'x' => {
                            mode.set_bits(0o0001 << index);
                        },
                        's' | 'S' => {
                            let bit = match i {
                                2 => 0o4000,
                                5 => 0o2000,
                                _ => unreachable!(),
                            };

                            mode.set_bits(bit);

                            if c == 's' {
                                mode.set_bits(0o0001 << index);
                            }
                        },
                        't' | 'T' => {
                            mode.set_bits(0o1000);

                            if c == 't' {
                                mode.set_bits(0o0001 << index);
                            }
                        },
                        '-' => {},
                        _   => unreachable!(),
                    };

                    index -= 1;
                }
            },
        };

        Ok(mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_from_octal() {
        let tests = vec![
            (0o000,  "000: ---------"),
            (0o644,  "644: rw-r--r--"),
            (0o755,  "755: rwxr-xr-x"),
            (0o0000, "000: ---------"),
            (0o0644, "644: rw-r--r--"),
            (0o0755, "755: rwxr-xr-x"),
            (0o1712, "1712: rwx--x-wT"),
            (0o1000, "1000: --------T"),
            (0o1001, "1001: --------t"),
            (0o2000, "2000: -----S---"),
            (0o2010, "2010: -----s---"),
            (0o4000, "4000: --S------"),
            (0o4100, "4100: --s------"),
            (0o7666, "7666: rwSrwSrwT"),
            (0o7777, "7777: rwsrwsrwt"),
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
            ("000",       "000: ---------"),
            ("644",       "644: rw-r--r--"),
            ("755",       "755: rwxr-xr-x"),
            ("0000",      "000: ---------"),
            ("0111",      "111: --x--x--x"),
            ("0222",      "222: -w--w--w-"),
            ("0333",      "333: -wx-wx-wx"),
            ("0444",      "444: r--r--r--"),
            ("0555",      "555: r-xr-xr-x"),
            ("0644",      "644: rw-r--r--"),
            ("0666",      "666: rw-rw-rw-"),
            ("0755",      "755: rwxr-xr-x"),
            ("0777",      "777: rwxrwxrwx"),
            ("1712",      "1712: rwx--x-wT"),
            ("1000",      "1000: --------T"),
            ("2000",      "2000: -----S---"),
            ("4000",      "4000: --S------"),
            ("1001",      "1001: --------t"),
            ("2010",      "2010: -----s---"),
            ("4100",      "4100: --s------"),
            ("7666",      "7666: rwSrwSrwT"),
            ("7777",      "7777: rwsrwsrwt"),
            ("755 ",      "755: rwxr-xr-x"),
            (" 755",      "755: rwxr-xr-x"),
            (" 755 ",     "755: rwxr-xr-x"),
            ("rwxrwxrwx", "777: rwxrwxrwx"),
            ("rwSrwSrwT", "7666: rwSrwSrwT"),
            ("rwsrwsrwt", "7777: rwsrwsrwt"),
        ];

        for test in tests {
            let input    = test.0;
            let expected = test.1;
            let mode     = Mode::from_str(input).unwrap().to_string();

            assert_eq!(expected, mode)
        }
    }
}
