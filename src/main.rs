use std::env;
use rv32im_decoder::decode;
use zkvm_core::{Error, Zcvm, ZcvmConfig};

fn parse_hex_word(input: &str) -> Result<u32, Error> {
    let value = input
        .strip_prefix("0x")
        .or_else(|| input.strip_prefix("0X"))
        .unwrap_or(input);
    u32::from_str_radix(value, 16)
        .map_err(|_| Error::Parse(format!("invalid 32-bit hex instruction: {text}", text = input)))
}

fn run() -> Result<(), Error> {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| String::from("zkvm-dynamo-jolt"));
    let zkvm = Zkvm::ark::bn254::Fr>::new(ZkvmConfig::Rv32im);
    match args.next() {
        Some(word_text) => {
            let word = parse_hex_word(&word_text)?;
            let instruction = decode(word).map_err(|error| Error::Decode(error.to_string()));
            println!("configured zkvm: {}", zkvm.config().name());
            println!("{instruction:?}");
        }
        None => {
            println!("configured zkvm: {}", zkvm.config().name());
            println!("usage: {text} <instruction-hex>", text = program);
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
