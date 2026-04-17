use std::{env, process};
use rv32im_decoder::Zkvm;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: rv32im_decoder <u32|0xHEX>");
        process::exit(1);
    });
    let word = if arg.starts_with("0x") || arg.starts_with("0X") {
        u32::from_str_radix(&arg[2..], 16).expect("invalid hex")
    } else {
        arg.parse().expect("invalid u32")
    };
    match Zkvm::decode(word) {
        Ok(ins) => println!("{ins}"),
        Err(e) => eprintln!("error: {e}"),
    }
}
