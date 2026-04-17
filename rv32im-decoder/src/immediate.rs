pub const fn sign_extend(value: u32, width: u8) -> i32 {
    if width == 0 {
        0
    } else {
        let shift = 32 - width as u32;
        ((value << shift) as i32) >> shift
    }
}

pub const fn i_type(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

pub const fn s_type(word: u32) -> i32 {
    let imm = ((word >> 25) << 5) | ((word >> 7) & 0x1f);
    sign_extend(imm, 12)
}

pub const fn b_type(word: u32) -> i32 {
    let imm = (((word >> 31) & 0x1) << 12)
        | (((word >> 7) & 0x1) << 11)
        | (((word >> 25) & 0x3f) << 5)
        | (((word >> 8) & 0x0f) << 1);
    sign_extend(imm, 13)
}

pub const fn u_type(word: u32) -> u32 {
    word & 0xfffff000
}

pub const fn j_type(word: u32) -> i32 {
    let imm = (((word >> 31) & 0x1) << 20)
        | (((word >> 12) & 0xff) << 12)
        | (((word >> 20) & 0x1) << 11)
        | (((word >> 21) & 0x03ff) << 1);
    sign_extend(imm, 21)
}

pub const fn shamt(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

pub const fn csr(word: u32) -> u16 {
    ((word >> 20) & 0x0fff) as u16
}

pub const fn zimm(word: u32) -> u8 {
    ((word >> 15) & 0x1f) as u8
}
