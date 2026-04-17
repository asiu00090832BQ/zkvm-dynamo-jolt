use crate::types::Zkvm;

pub fn execute_m(_: &mut Zcvm) {}
pub fn hierarchical_mul_u64(a: u32, b: u32) -> u64 {
    let a0 = a & 0xffff;
    let a1 = a >> 16;
    let b0 = b & 0xffff;
    let b1 = b >> 16;
    ((a1 * b1) as u64 << 32) + ((a1 * b0 + a0 * b1) as u64 << 16) + (a0 * b0) as u64
}
