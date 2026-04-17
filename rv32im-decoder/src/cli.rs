#![cfg(feature = "std")]

use std::{env, process::ExitCode};

pub fn run() -> ExitCode {
    match run_inner() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(1)
        }
    }
}

fn run_inner() -> Result<(), String> {
    let mut args = env::args().skip(1).peekable();

    if args.peek().is_none() {
        return Err(usage());
    }

    for arg in args {
        if arg == "-h" || arg == "--help" {
            println!("{}", usage());
            return Ok(());
        }

        let word = parse_word(&arg)?;
        let instruction = crate::decode(word).map_err(|e| format!("{arg}: {e}"))?;
        println!("{word:#010x}: {instruction}");
    }

    Ok(())
}

fn usage() -> String {
    "usage: rv32im-decoder <word> [<word> ...]\n\
     accepts decimal, 0x-prefixed hexadecimal, 0o-prefixed octal, and 0b-prefixed binary"
        .to_owned()
}

fn parse_word(input: &str) -> Result<u32, String> {
    let normalized: String = input.chars().filter(|&c| c != '_').collect();

    let (radix, digits) = if let Some(rest) = normalized
        .strip_prefix("0x")
        .or_else(|| normalized.strip_prefix("0X"))
    {
        (16, rest)
    } else if let Some(rest) = normalized
        .strip_prefix("0b")
        .or_else(|| normalized.strip_prefix("0B"))
    {
        (2, rest)
    } else if let Some(rest) = normalized
        .strip_prefix("0o")
        .or_else(|| normalized.strip_prefix("0O"))
    {
        (8, rest)
    } else {
        (10, normalized.as_str())
    };

    u32::from_str_radix(digits, radix)
        .map_err(|_| format!("failed to parse instruction word: {input}"))
}
