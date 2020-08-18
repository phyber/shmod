//
use std::env;
use std::fmt;
use std::ops::Range;

// Hold a mode
struct Mode(usize);

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
const BITS_RANGE: Range<usize> = 2..3; //Range { start: 2, end: 3 };

impl Mode {
    fn new(mode: usize) -> Self {
        Self(mode)
    }

    // Takes a string and converts it to octal
    // Can fail if numbers are over 7
    fn from_str(input: &str) -> Result<Self, String> {
        let mut output: usize = 0;

        // iterate over string
        for c in input.chars() {
            let i: usize = c.to_string().parse().unwrap();

            if i > OCTAL_MAX {
                return Err("over 7".to_owned());
            }

            output = (output << OCTAL_DIGIT_SIZE) | i;
        }

        Ok(Self::new(output))
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

fn main() {
    let args: Vec<String> = env::args().collect();

    let mode = if args.len() > 1 {
        &args[1]
    }
    else {
        eprintln!("{}", "Provide a mode");
        ::std::process::exit(1);
    };

    let mode = Mode::from_str(mode)
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            ::std::process::exit(1);
        });

    println!("{}", mode);
}
