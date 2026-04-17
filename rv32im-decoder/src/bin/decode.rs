use std::env;
use rv32im_decoder::decode;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: decode <hex_word>");
        std::process::exit(1);
    }
    let word = u32::from_str_radix(args[1].trim_start_matches("0x"), 16).unwrap();
    match decode(word) {
        Ok(inst) => println!"{:}", inst),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}
