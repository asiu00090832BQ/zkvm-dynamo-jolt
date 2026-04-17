use std::{env, process};

use rv32im_decoder::Zkvm;

fn parse_word(raw: &str) -> Result<u32, String> {
    let raw = raw.trim();

    if let Some(hex) = raw.strip_prefix("0x").or_else(|| raw.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16).map_err(|_| format!("invalid hex machine word: {raw}"))
    } else {
        raw.parse::<u32>()
            .map_err(|_| format!("invalid machine word: {raw}"))
    }
}

fn main() {
    let program = env::args()
        .next()
        .unwrap_or_else(|| "rv32im-decoder".to_owned());

    let Some(arg) = env::args().nth(1) else {
        eprintln!("usage: {program} <u32|0xHEX>");
        process::exit(2);
    };

    let word = match parse_word(&arg) {
        Ok(word) => word,
        Err(err) => {
            eprintln!("{err}");
            process::exit(2);
        }
    };

    match Zkvm::new().decode(word) {
        Ok(instruction) => println!("{instruction}"),
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    }
}
