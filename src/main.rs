#![feature(box_into_inner)]

mod interpreter;

use clap::{CommandFactory, Parser};
use interpreter::{core, utils};
use once_cell::sync::Lazy;
use std::{fs, process::exit};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_parser)]
    strip_illegal: Option<Option<bool>>,

    #[arg(short, long)]
    code: Option<String>,

    filename: Option<String>,
}

fn main() {
    let input_code: Lazy<String> = Lazy::new(|| {
        let runtime_args = Args::parse();

        let init_input_code = match runtime_args.code {
            Some(code) => code,
            None => match runtime_args.filename {
                Some(filename) => fs::read_to_string(filename).unwrap(),
                None => {
                    if let Err(_) = Args::command().print_help() {
                        utils::throw_err("CLI", "failed to display help menu")
                    }

                    exit(0);
                }
            },
        };
        let to_strip_illegal = match runtime_args.strip_illegal {
            Some(val) => match val {
                Some(bool_val) => bool_val,
                None => true,
            },
            None => false,
        };

        if to_strip_illegal {
            return init_input_code;
        } else {
            return utils::strip_code(&init_input_code);
        }
    });

    core::BrainfuckInstance::new().load_string((&*input_code).to_owned());
}
