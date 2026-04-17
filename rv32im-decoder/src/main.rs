use std::env;
use std::process::ExitCode;

fn parse_word(input: &str) -> Result<u32, String> {
    let trimmed = input.trim();

    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        return u32::from_str_radix(hex, 16).map_err(|e| e.to_string());
    }

    if let Some(bin) = trimmed
        .strip_prefix("0b")
        .or_else(|| trimmed.strip_prefix("0B"))
    {
        return u32::from_str_radix(bin, 2).map_err(|e| e.to_string());
    }

    trimmed
        .parse::<u32>()
        .or_else(|_| u32::from_str_radix(trimmed, 16))
        .map_err(|e| e.to_string())
}

fn print_usage(program: &str) {
    eprintln!("usage: {program} <instruction-word>");
    eprintln!("example: {program} 0x00c585b3");
}

fn main() -> ExitCode {
    let mut args = env::args();
    let program = args
        .next()
        .unwrap_or_else(|| "rv32im-decoder".to_string());

    let Some(arg) = args.next() else {
        print_usage(&program);
        return ExitCode::from(1);
    };

    if arg == "--help" || arg == "-h" {
        println!("usage: {program} <instruction-word>");
        println!("example: {program} 0x00c585b3");
        return ExitCode::SUCCESS;
    }

    let word = match parse_word(&arg) {
        Ok(word) => word,
        Err(err) => {
            eprintln!("failed to parse instruction word: {err}");
            return ExitCode::from(2);
        }
    };

    match rv32im_decoder::decode(word) {
        Ok(instr) => {
            println!("0x{word:08x} => {instr:?}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("decode error for 0x{word:08x}: {err}");
            ExitCode::from(3)
        }
    }
}
