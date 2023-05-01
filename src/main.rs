mod interpreter;

use std::fs;

use interpreter::{core, utils};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    strip_illegal: Option<bool>,

    #[arg(short, long)]
    code: Option<String>,

    filename: Option<String>
    
}

fn main() {
    let runtime_args = Args::parse();
    let mut input_code: String;

    input_code = match runtime_args.code {
        Some(code) => code,
        None => match runtime_args.filename {
            Some(filename) => fs::read_to_string(filename).unwrap(),
            None => utils::throw_err("CLI", "no input supplied")
        }
    };

    let to_strip_illegal = match runtime_args.strip_illegal {
        Some(val) => val,
        None => false
    };

    if to_strip_illegal {
        input_code = utils::strip_code(input_code.as_str());
    }

    

    core::BrainfuckInstance::new().load_string(input_code); // should be [ 0, 1, 1 ]
}
