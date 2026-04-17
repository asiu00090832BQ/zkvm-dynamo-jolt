pub fn opcode(word: u32) -> u32 { word & 0x7f }
pub fn rd(word: u32) -> usize { ((word >> 7) & 0x1f) as usize }
pub fn rs1(word: u32) -> usize { ((word >> 15) & 0x1f) as usize }
pub fn rs2(word: u32) -> usize { ((word >> 20) & 0x1f) as usize }
pub fn funct3(word: u32) -> u32 { (word >> 12) & 0x7 }
pub fn funct7(word: u32) -> u32 { (word >> 25) & 0x7f }
pub fn i_imm(word: u32) -> i32 { (word as i32) >> 20 }
pub fn u_imm(word: u32) -> i32 { (word & 0xfffff000) as i32 }
pub fn j_imm(word: u32) -> i32 {
    let i20 = (word >> 31) & 0x1;
    let i1_10 = (word >> 21) & 0x3ff;
    let i11 = (word >> 20) & 0x1;
    let i12_19 = (word >> 12) & 0xff;
    let imm = (i20 << 20) | (i12_19 << 12) | (i11 << 11) | (i1_10 << 1);
    if i20 == 1 { (imm | 0xffe00000) as i32 } else { imm as i32 }
}
