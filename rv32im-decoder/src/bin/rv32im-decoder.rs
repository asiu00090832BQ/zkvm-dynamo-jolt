#![cfg(feature = "std")]
use rv32im_decoder::decode;
fn main() {
    let arg = std::env::args().nth(1).expect("usage: rv32im-decoder <word>");
    let word = u32::from_str_radix(arg.trim_start_matches("0x"), 16).expect("invalid hex");
    println!("{:?}", decode(word).unwrap());
}
