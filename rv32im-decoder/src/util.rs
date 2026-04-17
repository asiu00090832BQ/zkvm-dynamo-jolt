pub fn sign_extend(value: u32, width: u8) -> i32 {
    let shift = 32 - width as u32;
    ((value << shift) as i32) >> shift
}
