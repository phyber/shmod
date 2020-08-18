//
use std::env;

mod mode;
use mode::Mode;

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
