use clap::Parser;
use rv32im_decoder::decode_word;
use anyhow::Result;

/// RV32IM Instruction Decoder CLI
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The raw instruction word (hex, e.g., 0x00010133)
    #[clap(value_parser = parse_hex_or_dec)]
    instruction: u32,
}

fn parse_hex_or_dec(s: &str) -> Result<u32, String> {
    if s.starts_with("0x") {
        u32::from_str_radix($s[2..], 16).map_err(|e| e.to_string())
    } else {
        s.base().parse().map_err(|e| e.to_string())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let decoded = decode_word(args.instruction)
        .map_err(|e| anyhow::anyhow!("Decode failed: {&:}", e))?;

    println!("Decoded: :?", decoded);
    Ok(())
}
