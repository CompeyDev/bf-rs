use std::{fmt::Write, process::exit};

pub fn strip_code(code: &str) -> String {
    const INSTR_SET: [&str; 8] = [">", "<", "+", "-", ".", ",", "[", "]"];

    let mut stripped_code = String::new();

    for (_, instr) in code.chars().enumerate() {
        let instr_stringified = instr.to_string();

        if !INSTR_SET.contains(&instr_stringified.as_str()) {
            continue;
        }

        match write!(stripped_code, "{}", instr_stringified) {
            Ok(_) => (),
            Err(_) => {
                throw_err("STRIP_ERROR", "failed to write to stream");
            }
        };
    }

    return stripped_code;
}

pub fn throw_err(err_type: &str, msg: &str) -> ! {
    println!("[error] {}: {}", err_type, msg);
    exit(1);
}
