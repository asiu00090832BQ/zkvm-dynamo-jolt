pub fn sext(val: u32, bits: u32) -> u32 {
    let m = 1 << (bits - 1);
    (val ^ m).wrapping_subm)
}
