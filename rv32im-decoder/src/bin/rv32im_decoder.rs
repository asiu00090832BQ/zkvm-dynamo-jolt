use rv32im_decoder::decode;

fn parse_word(text: &str) -> Result<u32, String> {
    let cleaned = text.trim().replace('_', "");
    if let Some(hex) = cleaned
        .strip_prefix("0x")
        .or_else(|| cleaned.strip_prefix("0X"))
    {
        u32::from_str_radix(hex, 16).map_err(|e| e.to_string())
    } else {
        cleaned.parse::<u32>().map_err(|e| e.to_string())
    }
}

fn main() {
    let Some(arg) = std::env::args().nth(1) else {
        eprintln!("usage: rv32im_decoder <instruction-word>");
        std::process::exit(1);
    };

    let word = match parse_word(&arg) {
        Ok(word) => word,
        Err(err) => {
            eprintln!("failed to parse instruction word: {err}");
            std::process::exit(1);
        }
    };

    match decode(word) {
        Ok(decoded) => {
            println!("word:     0x{:08x}", decoded.word);
            println!("mnemonic: {}", decoded.instruction.mnemonic());
            println!("decoded:  {:?}", decoded.instruction);
        }
        Err(err) => {
            eprintln!("decode error: {err}");
            std::process::exit(1);
        }
    }
}
