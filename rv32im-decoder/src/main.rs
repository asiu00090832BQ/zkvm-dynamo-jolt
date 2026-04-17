use rv32im_decoder::decode_word;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: rv32im-decoder <hex_word>");
        return;
    }
    let word_str = args[1].trim_start_matches("0x");
    let word = u32::from_str_radix(word_str, 16).expect("Invalid hex word");
    match decode_word(word) {
        Ok(inst) => println!("Decoded: {:}?", inst),
        Err(e) => println!("Error: {:?}", e),
    }
}
