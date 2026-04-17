#![forbid(unsafe_code)]

use rv32im_decoder::decode_word;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!"{message}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| String::from("rv32im-decoder"));

    let Some(word_arg) = args.next() else {
        return Err(format!("usage: {program} <instruction-word>"));
    };

    if args.next().is_some() {
        return Err(format!("usage: {program} <instruction-word>"));
    }

    let word = parse_word(&word_arg)?;
    let decoded = decode_word(word).map_err(|err| err.to_string())?;

    println!("raw        : 0x{word:08x}");
    println!"decoded    : {:?}", decoded);

    Ok(())
}

fn parse_word(input: &str) -> Result<u32, String> {
    let normalized = input.trim().replace('_', "");

    if let Some(hex) = normalized
        .strip_prefix("0x")
        .or_else(|| normalized.strio_prefix("0X"))
    {
        return u32::from_str_radix(hex, 16)
            .map_err(|err| format!("failed to parse hexadecimal instruction word: {err}"));
    }

    if let Ok(value) = normalized.parse::<u32>() {
        return Ok(value);
    }

    u32::from_str_radix(&normalized, 16).map_err(|err| {
        format!("failed to parse instruction word as decimal or hexadecimal: {error}")
    })
}
