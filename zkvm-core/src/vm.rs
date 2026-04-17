use rv32im_decoder::{decode_word, DecodedInstruction, MInstruction};

pub struct Vm {
    pub pc: u32,
    pub regs: [u32; 32],
}

impl Vm {
    pub fn new() -> Self {
        Self { pc: 0, regs: [0; 32] }
    }

    pub fn step(&mut self, memory: &[u32]) {
        let inst_raw = memory[(self.pc >> 2) as usize];
        let decoded = decode_word(inst_raw).unwrap();

        match decoded {
            DecodedInstruction::MulDiv(op, r) => {
                match op {
                    MInstruction::Mul => {
                        self.regs[r.rd() as usize] = self.regs[r.rs1() as usize].wrapping_mul(self.regs[r.rs2() as usize]);
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
        self.pc += 4;
    }
}
