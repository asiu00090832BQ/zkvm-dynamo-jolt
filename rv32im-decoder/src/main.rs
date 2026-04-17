use std::env;
use std::process;

use rv32im_decoder::{decode, ZkvmError};

fn parse_word(input: &str) -> Result<u32, ZkvmError> {
    let normalized = input.trim().replace('_', "");
    let trimmed = normalized
        .strip_prefix("0x")
        .or_else(|| normalized.strip_prefix("0X"))
        .unwrap_or(&normalized);

    u32::from_str_radix(trimmed, 16).map_err(|_| {
        ZkvmError::ParseError(format!(
            "failed to parse instruction word '{input}' as hexadecimal u32"
        ))
    })
}

fn main() {
    let mut args = env::args();
    let program = args
        .next()
        .unwrap_or_else(|| String::from("rv32im-decoder"));

    let arg = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("usage: {program} <instruction-word-hex>");
            process::exit(1);
        }
    };

    let word = match parse_word(&arg) {
        Ok(word) => word,
        Err(err) => {
            eprintln!("{err}");
            process::exit(2);
        }
    };

    match decode(word) {
        Ok(instruction) => {
            println!("{word:#010x}: {instruction}");
            println!("{instruction:?}");
        }
        Err(err) => {
            eprintln!("{err}");
            process::exit(3);
        }
    }
}
