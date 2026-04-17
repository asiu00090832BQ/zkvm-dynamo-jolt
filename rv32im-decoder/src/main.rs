use rv32im_decoder::decode;
fn main() {
    let word = 0x00100093; // addi x1, x0, 1
    match decode(word) {
        Ok(inst) => println!("Decoded: {:?}", inst),
        Err(e) => eprintln!("Error: {}", e),
    }
}