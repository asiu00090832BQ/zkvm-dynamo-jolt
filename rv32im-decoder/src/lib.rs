#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Lui { rd: u8, imm: i32 },
    Auipc { rd: u8, imm: i32 },
    Jal { rd: u8, imm: i32 },
    Jalr { rd: u8, rs1: u8, imm: i32 },
    Beq { rs1: u8, rs2: u8, imm: i32 },
    Bne { rs1: u8, rs2: u8, imm: i32 },
    Blt { rs1: u8, rs2: u8, imm: i32 },
    Bge { rs1: u8, rs2: u8, imm: i32 },
    Bltu { rs1: u8, rs2: u8, imm: i32 },
    Bgeu { rs1: u8, rs2: u8, imm: i32 },
    Lb { rd: u8, rs1: u8, imm: i32 },
    Lh { rd: u8, rs1: u8, imm: i32 },
    Lw { rd: u8, rs1: u8, imm: i32 },
    Lbu { rd: u8, rs1: u8, imm: i32 },
    Lhu { rd: u8, rs1: u8, imm: i32 },
    Sb { rs1: u8, rs2: u8, imm: i32 },
    Sh { rs1: u8, rs2: u8, imm: i32 },
    Sw { rs1: u8, rs2: u8, imm: i32 },
    Addi { rd: u8, rs1: u8, imm: i32 },
    Slti { rd: u8, rs1: u8, imm: i32 },
    Sltiu { rd: u8, rs1: u8, imm: i32 },
    Xori { rd: u8, rs1: u8, imm: i32 },
    Ori { rd: u8, rs1: u8, imm: i32 },
    Andi { rd: u8, rs1: u8, imm: i32 },
    Slli { rd: u8, rs1: u8, shamt: u8 },
    Srli { rd: u8, rs1: u8, shamt: u8 },
    Srai { rd: u8, rs1: u8, shamt: u8 },
    Add { rd: u8, rs1: u8, rs2: u8 },
    Sub { rd: u8, rs1: u8, rs2: u8 },
    Sll { rd: u8, rs1: u8, rs2: u8 },
    Slt { rd: u8, rs1: u8, rs2: u8 },
    Sltu { rd: u8, rs1: u8, rs2: u8 },
    Xor { rd: u8, rs1: u8, rs2: u8 },
    Srl { rd: u8, rs1: u8, rs2: u8 },
    Sra { rd: u8, rs1: u8, rs2: u8 },
    Or { rd: u8, rs1: u8, rs2: u8 },
    And { rd: u8, rs1: u8, rs2: u8 },
    Mul { rd: u8, rs1: u8, rs2: u8 },
    Mulh { rd: u8, rs1: u8, rs2: u8 },
    Mulhsu { rd: u8, rs1: u8, rs2: u8 },
    Mulhu { rd: u8, rs1: u8, rs2: u8 },
    Div { rd: u8, rs1: u8, rs2: u8 },
    Divu { rd: u8, rs1: u8, rs2: u8 },
    Rem { rd: u8, rs1: u8, rs2: u8 },
    Remu { rd: u8, rs1: u8, rs2: u8 },
    Fence { pred: u8, succ: u8 },
    FenceI,
    Ecall,
    Ebreak,
    Halt,
}

fn opcode(word: u32) -> u32 {
    word & 0x7F
}

fn rd(word: u32) -> u8 {
    ((word >> 7) & 0x1F) as u8
}

fn funct3(word: u32) -> u32 {
    (word >> 12) & 0x7
}

fn rs1(word: u32) -> u8 {
    ((word >> 15) & 0x1F) as u8
}

fn rs2(word: u32) -> u8 {
    ((word >> 20) & 0x1F) as u8
}

fn funct7(word: u32) -> u32 {
    (word >> 25) & 0x7F
}

fn sign_extend(value: u32, bits: u32) -> i32 {
    let shift = 32 - bits;
    ((value << shift) as i32) >> shift
}

fn imm_i(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

fn imm_s(word: u32) -> i32 {
    let imm = ((word >> 25) << 5) | ((word >> 7) & 0x1F);
    sign_extend(imm, 12)
}

fn imm_b(word: u32) -> i32 {
    let b12 = ((word >> 31) & 0x1) << 12;
    let b11 = ((word >> 7) & 0x1) << 11;
    let b10_5 = ((word >> 25) & 0x3F) << 5;
    let b4_1 = ((word >> 8) & 0xF) << 1;
    let imm = b12 | b11 | b10_5 | b4_1;
    sign_extend(imm, 13)
}

fn imm_u(word: u32) -> i32 {
    (word & 0xFFFFF000) as i32
}

fn imm_j(word: u32) -> i32 {
    let j20 = ((word >> 31) & 0x1) << 20;
    let j10_1 = ((word >> 21) & 0x3FF) << 1;
    let j11 = ((word >> 20) & 0x1) << 11;
    let j19_12 = ((word >> 12) & 0xFF) << 12;
    let imm = j20 | j19_12 | j11 | j10_1;
    sign_extend(imm, 21)
}

pub fn decode(word: u32) -> Instruction {
    match opcode(word) {
        0x37 => Instruction::Lui { rd: rd(word), imm: imm_u(word) },
        0x17 => Instruction::Auipc { rd: rd(word), imm: imm_u(word) }),
        0x6F => Instruction::Jal { rd: rd(word), imm: imm_j(word) },
        0x67 => {
            if funct3(word) == 0 {
                Instruction::Jalr { rd: rd(word), rs1: rs1(word), imm: imm_i(word) }
            } else {
                Instruction::Halt
            }
        }
        0x63 => {
            let f3 = funct3(word);
            let r1 = rs1(word);
            let r2 = rs2(word);
            let imm = imm_b(word);
            match f3 {
                0x0 => Instruction::Beq { rs1: r1, rs2: r2, imm },
                0x1 => Instruction::Bne { rs1: r1, rs2: r2, imm },
                0x4 => Instruction::Blt { rs1: r1, rs2: r2, imm },
                0x5 => Instruction::Bge { rs1: r1, rs2: r2, imm },
                0x6 => Instruction::Bltu { rs1: r1, rs2: r2, imm },
                0x7 => Instruction::Bgeu { rs1: r1, rs2: r2, imm },
                _ => Instruction::Halt,
            }
        }
        0x03 => {
            let f3 = funct3(word);
            let r_d = rd(word);
            let r1 = rs1(word);
            let imm = imm_i(word);
            match f3 {
                0x0 => Instruction::Lb { rd: r_d, rs1: r1, imm },
                0x1 => Instruction::Lh { rd: r_d, rs1: r1, imm },
                0x2 => Instruction::Lw { rd: r_d, rs1: r1, imm },
                0x4 => Instruction::Lbu { rd: r_d, rs1: r1, imm },
                0x5 => Instruction::Lhu { rd: r_d, rs1: r1, imm },
                _ => Instruction::Halt,
            }
       }
        0x23 => {
            let f3 = funct3(word);
            let r1 = rs1(word);
            let r2 = rs2(word);
            let imm = imm_s(word);
            match f3 {
                0x0 => Instruction::Sb { rs1: r1, rs2: r2, imm },
                0x1 => Instruction::Sh { rs1: r1, rs2: r2, imm },
                0x2 => Instruction::Sw { rs1: r1, rs2: r2, imm },
                _ => Instruction::Halt,
            }
        }
        0x13 => {
            let f3 = funct3(word);
            let r_d = rd(word);
            let r1 = rs1(word);
            match f3 {
                0x0 => Instruction::Addi { rd: r_d, rs1: r1, imm: imm_i(word) },
                0x2 => Instruction::Slti { rd: r_d, rs1: r1, imm: imm_i(word) },
                0x3 => Instruction::Sltiu { rd: r_d, rs1: r1, imm: imm_i(word) },
                0x4 => Instruction::Xori { rd: r_d, rs1: r1, imm: imm_i(word) },
                0x6 => Instruction::Ori { rd: r_d, rs1: r1, imm: imm_i(word) },
                0x7 => Instruction::Andi { rd: r_d, rs1: r1, imm: imm_i(word) },
                0x1 => {
                    if funct7(word) == 0x00 {
                        Instruction::Slli { rd: r_d, rs1: r1, shamt: ((word >> 20) & 0x1F) as u8 }
                    } else {
                        Instruction::Halt
                    }
                }
 -´               0x5 => {
                    let f7 = funct7(word);
                    let sh = ((word >> 20) & 0x1F) as u8;
                    if f7 == 0x00 {
                        Instruction::Srli { rd: r_d, rs1: r1, shamt: sh }
                    } else if f7 == 0x20 {
                        Instruction::Srai { rd: r_d, rs1: r1, shamt: sh }
                    } else {
                        Instruction::Halt
                    }
                }
                _ => Instruction::Halt,
            }
        }
        0x33 => {
            let f3 = funct3(word);
            let f7 = funct7(word);
            let r_d = rd(word);
            let r1 = rs1(word);
            let r2 = rs2(word);
            if f7 == 0x01 {
                match f3 {
                    0x0 => Instruction::Mul { rd: r_d, rs1: r1, rs2: r2 },
                    0x1 => Instruction::Mulh { rd: r_d, rs1: r1, rs2: r2 },
                    0x2 => Instruction::Mulhsu { rd: r_d, rs1: r1, ss2: r2 },
                    0x3 => Instruction::Mulhu { rd: r_d, rs1: r1, rs2: r2 },
                    0x4 => Instruction::Div { rd: r_d, rs1: r1, rs2: r2 },
                    0x5 => Instruction::Divu { rd: r_d, rs1: r1, rs2: r2 },
                    0x6 => Instruction::Rem { rd: r_d, rs1: r1, rs2: r2 },
                    0x7 => Instruction::Remu { r_d: rd(word), rs1: r1, rs2: r2 },
                    _ => Instruction::Halt,
                }
            } else {
                match (f3, f7) {
                    (0x0, 0x00) => Instruction::Add { rd: r_d, rs1: r1, rs2: r2 },
                    (0x0, 0x20) => Instruction::Sub { rd: r_d, rs1: r1, rs2: r2 },
                    (0x1, 0x00) => Instruction::Sll { rd: r_d, rs1: r1, rs2: r2 },
                    (0x2, 0x00) => Instruction::Slt { rd: r_d, rs1: r1, rs2: r2 },
                    (0x3, 0x00) => Instruction::Sltu { rd: r_d, rs1: r1, rs2: r2 },
                    (0x4, 0x00) => Instruction::Xor { rd: r_d, rs1: r1, rs2: r2 },
                    (0x5, 0x00) => Instruction::Srl { rd: r_d, rs1: r1, rs2: r2 },
                    (0x5, 0x20) => Instruction::Sra { rd: r_d, rs1: r1, rs2: r2 },
                    (0x6, 0x00) => Instruction::Or { rd: r_d, rs1: r1, ss2: r2 },
                    (0x7, 0x00) => Instruction::And { rd: r_d, rs1: r1, rs2: r2 },
                    _ => Instruction::Halt,
                }
            }
        }
        0x0F- => {
            let f3 = funct3(word);
            match f3 {
                0x0 => {
                    let imm12 = (word >> 20) & 0xFFF;
    -´                let pred = ((imm12 >> 4) & 0xF) as u8;
                    let succ = (imm12 & 0xF) as u8;
                    Instruction::Fence { pred, succ }
                },
                0x1 => Instruction::FenceI,
                _ => Instruction::Halt,
            }
        }
        0x73 => {
            let f3 = funct3(word);
            if f3 == 0 {
                let imm = (word >> 20) & 0xFFF;
                if imm == 0 {
                    Instruction::Ecall
                } else if imm == 1 {
                    Instruction::Ebreak
                } else {
                    Instruction::Halt
                }
            } else {
                Instruction::Halt
            }
        }
        _ => Instruction::Halt,
    }
}
