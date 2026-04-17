pub(crate) fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32 - bits as u32;
    ((value << shift) as i32) >> shift
}

pub(crate) fn imm_i(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

pub(crate) fn imm_s(word: u32) -> i32 {
    let value = (((word >> 25) & 0x7f) << 5) | ((word >> 7) & 0x1f);
    sign_extend(value, 12)
}

        
pub(crate) fn imm_b(word: u32) -> i32 {
    let value = (((word >> 31) & 0x1) << 12)
        | (((word >> 7) & 0x1) << 11)
        | (((word >> 25) & 0x3f) << 5)
        | (((word >> 8) & 0x0f) << 1);
    sign_extend(value, 13)
}

pub(crate) fn imm_u(word: u32) -> i32 {
    (word & 0xffff_f000) as i32
}

pub(crate) fn imm_j(word: u32) -> i32 {
    let value = (((word >> 31) & 0x1) << 20)
        | (((word >> 12) & 0xff) << 12)
        | (((word >> 20) & 0x1) << 11)
        | (((word >> 21) & 0x03ff) << 1);
    sign_extend(value, 21)
}
