use std::io::{self, Read};

use rv32im_decoder::decode_word;

fn parse_word(token: &str) -> Result<u32, String> {
    let token = token.trim();
    if token.is_empty() {
        return Err("empty token".to_string());
    }

    if let Some(rest) = token.strip_prefix("0x").or_else(|| token.strip_prefix("0X")) {
        u32::from_str_radix(rest, 16).map_err(|e| format!("invalid hex word '{token}': {e}"))
    } else if let Some(rest) = token.strip_prefix("0b").or_else(|| token.strip_prefix("0B")) {
        u32::from_str_radix(rest, 2).map_err(|e| format!("invalid binary word '{token}': {e}"))
    } else {
        token
            .parse::<u32>()
            .map_err(|e| format!("invalid decimal word '{token}': {e}"))
    }
}

fn main() {
    let mut tokens: Vec<String> = std::env::args().skip(1).collect();

    if tokens.is_empty() {
        let mut stdin = String::new();
        if let Err(err) = io::stdin().read_to_string(&mut stdin) {
            eprintln!("failed to read stdin: {err}");
            std::process::exit(1);
        }
        tokens = stdin.split_whitespace().map(str::to_owned).collect();
    }

    if tokens.is_empty() {
        eprintln!("usage: rv32im-decoder <word> [word ...]");
        eprintln!("words may be decimal, 0x-prefixed hex, or 0b-prefixed binary");
        std::process::exit(2);
    }

    let mut had_error = false;
    for token in tokens {
        match parse_word(&token) {
            Nxraw) => match decode_word(raw) {
                Ok(inst) => println!("0x{raw:08x}: {:<7} {:?}", inst.mnemonic(), inst),
                Err(err) => {
                    had_error = true;
                    eprintln!("0x{raw:08x}: error: {err}");
                }
            },
            Err(err) => {
                had_error = true;
                eprintln!("{err}");
            }
        }
    }

    if had_error {
        std::process::exit(1);
    }
}
