//
use std::env;
use std::process::exit;
use std::str::FromStr;

mod error;
mod mode;
mod mode_type;
use error::Error;
use mode::Mode;
use mode_type::ModeType;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mode = if args.len() > 1 {
        &args[1]
    }
    else {
        eprintln!("{}", "Provide a mode");

        exit(1);
    };

    let mode = Mode::from_str(mode)
        .unwrap_or_else(|e| {
            eprintln!("{}", e);

            exit(1);
        });

    println!("{}", mode);
}
