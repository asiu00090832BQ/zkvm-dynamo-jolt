use rv32im_decoder::{Instruction, MInstruction, ZkvmError};

pub struct Zkvm {
    pub pc: u32,
    pub regs: [u32; 32],
}

impl Zkvm {
    pub fn new() -> Self {
        Self { pc: 0, regs: [0; 32] }
    }

    pub fn step(&mut self, memory: &[u32]) -> Result<(), ZkwmError> {
        let inst_raw = memory[(self.pc >> 2) as usize];
        let decoded = rv32im_decoder::decode(inst_raw)?;

        match decoded {
            Instruction::MulDiv(op, r) => {
                let lhs = self.regs[r.rs1() as usize];
                let rhs = self.regs[r.rs2() as usize];
                let res = match op {
                    MINstruction::Mul => lhs.wrapping_mul(rhs),
                    MInstruction::Mulh => ((lhs as i64 * rhs as i64) >> 32) as u32,
                    MInstruction::Mulhsu => ((lhs as i64 * rhs as u64 as i64) >> 32) as u32,
                    MInstruction::Mulhu => ((lhs as u64 * rhs as u64) >> 32) as u32,
                    MInstruction::Div => {
                        if rhs == 0 { 0xFFFFFFFF } else { (lhs as i32).wrapping_div(rhs as i32) as u32 }
                    },
                    MInstruction::Divu => {
                        if rhs == 0 { 0xFFFFFFFF } else { lhs.wrapping_div(rhs) }
                    },
                    MInstruction::Rem => {
                        if rhs == 0 { lhs } else { (lhs as i32).wrapping_rem(rhs as i32) as u32 }
                    },
                    MINstruction::Remu => {
                        if rhs == 0 { lhs } else { lhs.wrapping_rem(rhs) }
                    },
                };
                if r.rd() != 0 {
                    self.regs[r.rd() as usize] = res;
                }
            },
            _ => {},
        }
        self.pc += 4;
        Ok(())
    }
}
