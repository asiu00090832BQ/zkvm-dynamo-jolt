use crate::{
    decode::decode_word,
    error::ZkvmError,
    types::{BranchKind, Instruction, Op, OpImm, Register},
};

#[derive(Debug, Clone)]
pub struct Zkvm {
    registers: [u32; 32],
    pc: u32,
}

impl Default for Zkvm {
    fn default() -> Self {
        Self::new()
    }
}

impl Zkvm {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            pc: 0,
        }
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
    }

    pub fn read_reg(&self, reg: Register) -> u32 {
        self.registers[reg.index()]
    }

    pub fn write_reg(&mut self, reg: Register, value: u32) {
        if reg.raw() != 0 {
            self.registers[reg.index()] = value;
        }
    }

    pub fn decode(&self, word: u32) -> Result<Instruction, ZkvmError> {
        decode_word(word)
    }

    pub fn step_word(&mut self, word: u32) -> Result<Instruction, ZkvmError> {
        let instruction = decode_word(word)?;
        self.execute(instruction)?;
        Ok(instruction)
    }

    pub fn execute(&mut self, instruction: Instruction) -> Result<(), ZkvmError> {
        let current_pc = self.pc;
        let next_pc = current_pc.wrapping_add(4);

        match instruction {
            Instruction::Lui { rd, imm } => {
                self.write_reg(rd, imm as u32);
                self.pc = next_pc;
            }
            Instruction::Auipc { rd, imm } => {
                self.write_reg(rd, current_pc.wrapping_add(imm as u32));
                self.pc = next_pc;
            }
            Instruction::Jal { rd, imm } => {
                self.write_reg(rd, next_pc);
                self.pc = current_pc.wrapping_add(imm as u32);
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let target = self.read_reg(rs1).wrapping_add(imm as u32) & !1;
                self.write_reg(rd, next_pc);
                self.pc = target;
            }
            Instruction::Branch { kind, rs1, rs2, imm } => {
                let lhs = self.read_reg(rs1);
                let rhs = self.read_reg(rs2);
                let taken = branch_taken(kind, lhs, rhs);
                self.pc = if taken {
                    current_pc.wrapping_add(imm as u32)
                } else {
                    next_pc
                };
            }
            Instruction::OpImm { kind, rd, rs1, imm } => {
                let lhs = self.read_reg(rs1);
                self.write_reg(rd, exec_op_imm(kind, lhs, imm));
                self.pc = next_pc;
            }
            Instruction::Op { kind, rd, rs1, rs2 } => {
                let lhs = self.read_reg(rs1);
                let rhs = self.read_reg(rs2);
                self.write_reg(rd, exec_op(kind, lhs, rhs));
                self.pc = next_pc;
            }
            Instruction::Load { .. } => {
                return Err(ZkvmError::UnsupportedInstruction {
                    word: 0,
                    reason: "memory loads are not implemented in Zkvm::execute",
                })
            }
            Instruction::Store { .. } => {
                return Err(ZkvmError::UnsupportedInstruction {
                    word: 0,
                    reason: "memory stores are not implemented in Zkvm::execute",
                })
            }
            Instruction::Fence => {
                self.pc = next_pc;
            }
            Instruction::Ecall => {
                return Err(ZkvmError::UnsupportedInstruction {
                    word: 0,
                    reason: "ecall traps are not implemented in Zkvm::execute",
                })
            }
            Instruction::Ebreak => {
                return Err(ZkvmError::UnsupportedInstruction {
                    word: 0,
                    reason: "ebreak traps are not implemented in Zkvm::execute",
                })
            }
        }

        Ok(())
    }
}

fn branch_taken(kind: BranchKind, lhs: u32, rhs: u32) -> bool {
    match kind {
        BranchKind::Beq => lhs == rhs,
        BranchKind::Bne => lhs != rhs,
        BranchKind::Blt => (lhs as i32) < (rhs as i32),
        BranchKind::Bge => (lhs as i32) >= (rhs as i32),
        BranchKind::Bltu => lhs < rhs,
        BranchKind::Bgeu => lhs >= rhs,
    }
}

fn exec_op_imm(kind: OpImm, lhs: u32, imm: i32) -> u32 {
    match kind {
        OpImm::Addi => lhs.wrapping_add(imm as u32),
        OpImm::Slti => ((lhs as i32) < imm) as u32,
        OpImm::Sltiu => (lhs < imm as u32) as u32,
        OpImm::Xori => lhs ^ (imm as u32),
        OpImm::Ori => lhs | (imm as u32),
        OpImm::Andi => lhs & (imm as u32),
        OpImm::Slli => lhs << ((imm as u32) & 0x1f),
        OpImm::Srli => lhs >> ((imm as u32) & 0x1f),
        OpImm::Srai => ((lhs as i32) >> ((imm as u32) & 0x1f)) as u32,
    }
}

fn exec_op(kind: Op, lhs: u32, rhs: u32) -> u32 {
    match kind {
        Op::Add => lhs.wrapping_add(rhs),
        Op::Sub => lhs.wrapping_sub(rhs),
        Op::Sll => lhs << (rhs & 0x1f),
        Op::Slt => ((lhs as i32) < (rhs as i32)) as u32,
        Op::Sltu => (lhs < rhs) as u32,
        Op::Xor => lhs ^ rhs,
        Op::Srl => lhs >> (rhs & 0x1f),
        Op::Sra => ((lhs as i32) >> (rhs & 0x1f)) as u32,
        Op::Or => lhs | rhs,
        Op::And => lhs & rhs,
        Op::Mul => lhs.wrapping_mul(rhs),
        Op::Mulh => (((lhs as i32 as i64) * (rhs as i32 as i64)) >> 32) as u32,
        Op::Mulhsu => (((lhs as i32 as i64) * (rhs as u64 as i64)) >> 32) as u32,
        Op::Mulhu => (((lhs as u64) * (rhs as u64)) >> 32) as u32,
        Op::Div => div_signed(lhs, rhs),
        Op::Divu => div_unsigned(lhs, rhs),
        Op::Rem => rem_signed(lhs, rhs),
        Op::Remu => rem_unsigned(lhs, rhs),
    }
}

fn div_signed(lhs: u32, rhs: u32) -> u32 {
    let lhs = lhs as i32;
    let rhs = rhs as i32;

    if rhs == 0 {
        u32::MAX
    } else if lhs == i32::MIN && rhs == -1 {
        lhs as u32
    } else {
        lhs.wrapping_div(rhs) as u32
    }
}

fn div_unsigned(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
        u32::MAX
    } else {
        lhs / rhs
    }
}

fn rem_signed(lhs: u32, rhs: u32) -> u32 {
    let lhs_i = lhs as i32;
    let rhs_i = rhs as i32;

    if rhs_i == 0 {
        lhs
    } else if lhs_i == i32::MIN && rhs_i == -1 {
        0
    } else {
        lhs_i.wrapping_rem(rhs_i) as u32
    }
}

fn rem_unsigned(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
        lhs
    } else {
        lhs % rhs
    }
}
