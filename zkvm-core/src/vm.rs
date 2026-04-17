use rv32im_decoder::decode::{decode_word, execute_rv32m};
use rv32im_decoder::isa::i::{Rv32I};
use rv32im_decoder::isa::m::Rv32M;
use rv32im_decoder::isa::{
    BTypeFields, ITypeFields, Instruction, JTypeFields, RTypeFields, Register, STypeFields,
    ShiftImmFields, UTypeFields,
};
use rv32im_decoder::{ZkvmError, ZkvmResult};

pub struct Zkvm<'a> {
    pc: u32,
    regs: [u32; 32],
    memory: &'a mut [u8],
}

impl<'a> Zkvm<'a> {
    pub fn new(memory: &'a mut [u8]) -> Self {
        Self {
            pc: 0,
            regs: [0; 32],
            memory,
        }
    }

    pub const fn pc(&self) -> u32 {
        self.pc
    }

    pub fn registers(&self) -> &[u32; 32] {
        &self.regs
    }

    pub fn registers_mut(&mut self) -> &mut [u32; 32] {
        &mut self.regs
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
    }

    pub fn read_reg(&self, reg: Register) -> u32 {
        self.regs[reg.index()]
    }

    pub fn write_reg(&mut self, reg: Register, value: u32) {
        if reg != Register::ZERO {
            self.regs[reg.index()] = value;
        }
    }

    pub fn fetch_word(&self) -> ZkvmResult<u32> {
        if (self.pc & 0x3) != 0 {
            return Err(ZkvmError::MisalignedInstruction { pc: self.pc });
        }

        let base = self.bounds_check(self.pc, 4)?;
        Ok(u32::from_le_bytes([
            self.memory[base],
            self.memory[base + 1],
            self.memory[base + 2],
            self.memory[base + 3],
        ]))
    }

    pub fn step(&mut self) -> ZkvmResult<()> {
        let word = self.fetch_word()?;
        let inst = decode_word(word)?;
        self.execute(inst)
    }

    pub fn execute(&mut self, inst: Instruction) -> ZkvmResult<()> {
        match inst {
            Instruction::I(inst) => self.execute_rv32i(inst),
            Instruction::M(inst) => self.execute_rv32m(inst),
        }?;

        self.regs[0] = 0;
        Ok(())
    }

    fn execute_rv32i(&mut self, inst: Rv32I) -> ZkvmResult<()> {
        match inst {
            Rv32I::Lui(fields) => {
                self.write_reg(fields.rd, fields.imm);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Rv32I::Auipc(fields) => {
                self.write_reg(fields.rd, self.pc.wrapping_add(fields.imm as u32));
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Rv32I::Jal(fields) => self.exec_jal(fields),
            Rv32I::Jalr(fields) => self.exec_jalr(fields),
            Rv32I::Beq(fields) => self.exec_branch(fields, |lhs, rhs| lhs == rhs),
            Rv32I::Bne(fields) => self.exec_branch(fields, |lhs, rhs| lhs != rhs),
            Rv32I::Blt(fields) => self.exec_branch(fields, |lhs, rhs| (lhs as i32) < (rhs as i32)),
            Rv32I::Bge(fields) => self.exec_branch(fields, |lhs, rhs| (lhs as i32) >= (rhs as i32)),
            Rv32I::Bltu(fields) => self.exec_branch(fields, |lhs, rhs| lhs < rhs),
            Rv32I::Bgeu(fields) => self.exec_branch(fields, |lhs, rhs| lhs >= rhs),
            Rv32I::Lb(fields) => self.exec_load(fields, 1, true),
            Rv32I::Lh(fields) => self.exec_load(fields, 2, true),
            Rv32I::Lw(fields) => self.exec_load(fields, 4, false),
            Rv32I::Lbu(fields) => self.exec_load(fields, 1, false),
            Rv32I::Lhu(fields) => self.exec_load(fields, 2, false),
            Rv32I::Sb(fields) => self.exec_store(fields, 1),
            Rv32I::Sh(fields) => self.exec_store(fields, 2),
            Rv32I::Sw(fields) => self.exec_store(fields, 4),
            Rv32I::Addi(fields) => {
                let value = self.read_reg(fields.rs1).wrapping_add(fields.imm as u32);
                self.write_reg(fields.rd, value);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Rv32I::Slti(fields) => {
                let lhs = self.read_reg(fields.rs1) as i32;
                self.write_reg(fields.rd, (lhs < fields.imm) as u32);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Rv32I::Sltiu(fields) => {
                let lhs = self.read_reg(fields.rs1);
                self.write_reg(fields.rd, (lhs < fields.imm as u32) as u32);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Rv32I::Xori(fields) => {
                let value = self.read_reg(fields.rs1) ^ fields.imm as u32;
                self.write_reg(fields.rd, value);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Rv32I::Ori(fields) => {
                let value = self.read_reg(fields.rs1) | fields.imm as u32;
                self.write_reg(fields.rd, value);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Rv32I::Andi(fields) => {
                let value = self.read_reg(fields.rs1) & fields.imm as u32;
                self.write_reg(fields.rd, value);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Rv32I::Slli(fields) => self.exec_slli(fields),
            Rv32I::Srli(fields) => self.exec_srli(fields),
            Rv32I::Srai(fields) => self.exec_srai(fields),
            Rv32I::Add(fields) => self.exec_add(fields),
            Rv32I::Sub(fields) => self.exec_sub(fields),
            Rv32I::Sll(fields) => self.exec_sll(fields),
            Rv32I::Slt(fields) => self.exec_slt(fields),
            Rv32I::Sltu(fields) => self.exec_sltu(fields),
            Rv32I::Xor(fields) => self.exec_xor(fields),
            Rv32I::Srl(fields) => self.exec_srl(fields),
            Rv32I::Sra(fields) => self.exec_sra(fields),
            Rv32I::Or(fields) => self.exec_or(fields),
            Rv32I::And(fields) => self.exec_and(fields),
            Rv32I::Fence => {
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Rv32I::FenceI => {
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Rv32I::Ecall(_) => Err(ZkvmError::Ecall),
            Rv32I::Ebreak => Err(ZkvmError::Ebreak),
        }
    }

    fn execute_rv32m(&mut self, inst: Rv32M) -> ZkvmResult<()> {
        let fields = inst.fields();
        let lhs = self.read_reg(fields.rs1);
        let rhs = self.read_reg(fields.rs2);
        let value = execute_rv32m(&inst, lhs, rhs);
        self.write_reg(fields.rd, value);
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn exec_jal(&mut self, fields: JTypeFields) -> ZkvmResult<()> {
        let next_pc = self.pc.wrapping_add(4);
        self.write_reg(fields.rd, next_pc);
        self.pc = self.pc.wrapping_add(fields.imm as u32);
        Ok(())
    }

    fn exec_jalr(&mut self, fields: ITypeFields) -> ZkvmResult<()> {
        let next_pc = self.pc.wrapping_add(4);
        let base = self.read_reg(fields.rs1);
        self.write_reg(fields.rd, next_pc);
        self.pc = base.wrapping_add(fields.imm as u32) & !1;
        Ok(())
    }

    fn exec_branch<F>(&mut self, fields: BTypeFields, predicate: F) -> ZkvmResult<()>
    where
        F: FnOnce(u32, u32) -> bool,
    {
        let lhs = self.read_reg(fields.rs1);
        let rhs = self.read_reg(fields.rs2);

        if predicate(lhs, rhs) {
            self.pc = self.pc.wrapping_add(fields.imm as u32);
        } else {
            self.pc = self.pc.wrapping_add(4);
        }

        Ok(())
    }

    fn exec_load(&mut self, fields: ITypeFields, size: usize, sign_ext: bool) -> ZkvmResult<()> {
        let address = self.read_reg(fields.rs1).wrapping_add(fields.imm as u32);
        let value = match size {
            1 => {
                let raw = self.load8(address)?;
                if sign_ext {
                    (raw as i8 as i32) as u32
                } else {
                    raw as u32
                }
            }
            2 => {
                let raw = self.load16(address)?;
                if sign_ext {
                    (raw as i16 as i32) as u32
                } else {
                    raw as u32
                }
            }
            4 => self.load32(address)?,
            _ => return Err(ZkvmError::UnsupportedInstruction { word: 0 }),
        };

        self.write_reg(fields.rd, value);
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn exec_store(&mut self, fields: STypeFields, size: usize) -> ZkvmResult<()> {
        let address = self.read_reg(fields.rs1).wrapping_add(fields.imm as u32);
        let value = self.read_reg(fields.rs2);

        match size {
            1 => self.store8(address, value as u8)?,
            2 => self.store16(address, value as u16)?,
            4 => self.store32(address, value)?,
            _ => return Err(ZkvmError::UnsupportedInstruction { word: 0 }),
        }

        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn exec_slli(&mut self, fields: ShiftImmFields) -> ZkvmResult<()> {
        self.write_reg(fields.rd, self.read_reg(fields.rs1) << fields.shamt);
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn exec_srli(&mut self, fields: ShiftImmFields) -> ZkvmResult<()> {
        self.write_reg(fields.rd, self.read_reg(fields.rs1) >> fields.shamt);
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn exec_srai(&mut self, fields: ShiftImmFields) -> ZkvmResult<()> {
        self.write_reg(fields.rd, ((self.read_reg(fields.rs1) as i32) >> fields.shamt) as u32);
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn exec_add(&mut self, fields: RTypeFields) -> ZkvmResult<()> {
        self.write_reg(
            fields.rd,
            self.read_reg(fields.rs1).wrapping_add(self.read_reg(fields.rs2)),
        );
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn exec_sub(&mut self, fields: rv32im_decoder::isa::i::Sub) -> ZkvmResult<()> {
        self.write_reg(
            fields.rd,
            self.read_reg(fields.rs1).wrapping_sub(self.read_reg(fields.rs2)),
        );
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn exec_sll(&mut self, fields: RTypeFields) -> ZkvmResult<()> {
        let shamt = self.read_reg(fields.rs2) & 0x1f;
        self.write_reg(fields.rd, self.read_reg(fields.rs1) << shamt);
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn exec_slt(&mut self, fields: RTypeFields) -> ZkvmResult<()> {
        self.write_reg(
            fields.rd,
            ((self.read_reg(fields.rs1) as i32) < (self.read_reg(fields.rs2) as i32)) as u32,
        );
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn exec_sltu(&mut self, fields: RTypeFields) -> ZkvmResult<()> {
        self.write_reg(fields.rd, (self.read_reg(fields.rs1) < self.read_reg(fields.rs2)) as u32);
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn exec_xor(&mut self, fields: RTypeFields) -> ZkvmResult<()> {
        self.write_reg(fields.rd, self.read_reg(fields.rs1) ^ self.read_reg(fields.rs2));
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn exec_srl(&mut self, fields: RTypeFields) -> ZkvmResult<()> {
        let shamt = self.read_reg(fields.rs2) & 0x1f;
        self.write_reg(fields.rd, self.read_reg(fields.rs1) >> shamt);
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn exec_sra(&mut self, fields: RTypeFields) -> ZkvmResult<()> {
        let shamt = self.read_reg(fields.rs2) & 0x1f;
        self.write_reg(fields.rd, ((self.read_reg(fields.rs1) as i32) >> shamt) as u32);
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn exec_or(&mut self, fields: RTypeFields) -> ZkvmResult<()> {
        self.write_reg(fields.rd, self.read_reg(fields.rs1) | self.read_reg(fields.rs2));
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn exec_and(&mut self, fields: RTypeFields) -> ZkvmResult<()> {
        self.write_reg(fields.rd, self.read_reg(fields.rs1) & self.read_reg(fields.rs2));
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn load8(&self, address: u32) -> ZkvmResult<u8> {
        let index = self.bounds_check(address, 1)?;
        Ok(self.memory[index])
    }

    fn load16(&self, address: u32) -> ZkvmResult<u16> {
        if (address & 0x1) != 0 {
            return Err(ZkvmError::MisalignedLoad { address, size: 2 });
        }

        let index = self.bounds_check(address, 2)?;
        Ok(u16::from_le_bytes([self.memory[index], self.memory[index + 1]]))
    }

    fn load32(&self, address: u32) -> ZkvmResult<u32> {
        if (address & 0x3) != 0 {
            return Err(ZkvmError::MisalignedLoad { address, size: 4 });
        }

        let index = self.bounds_check(address, 4)?;
        Ok(u32::from_le_bytes([
            self.memory[index],
            self.memory[index + 1],
            self.memory[index + 2],
            self.memory[index + 3],
        ]))
    }

    fn store8(&mut self, address: u32, value: u8) -> ZkvmResult<()> {
        let index = self.bounds_check(address, 1)?;
        self.memory[index] = value;
        Ok(())
    }

    fn store16(&mut self, address: u32, value: u16) -> ZkvmResult<()> {
        if (address & 0x1) != 0 {
            return Err(ZkvmError::MisalignedStore { address, size: 2 });
        }

        let index = self.bounds_check(address, 2)?;
        let bytes = value.to_le_bytes();
        self.memory[index] = bytes[0];
        self.memory[index + 1] = bytes[1];
        Ok(())
    }

    fn store32(&mut self, address: u32, value: u32) -> ZkvmResult<()> {
        if (address & 0x3) != 0 {
            return Err(ZkvmError::MisalignedStore { address, size: 4 });
        }

        let index = self.bounds_check(address, 4)?;
        let bytes = value.to_le_bytes();
        self.memory[index] = bytes[0];
        self.memory[index + 1] = bytes[1];
        self.memory[index + 2] = bytes[2];
        self.memory[index + 3] = bytes[3];
        Ok(())
    }

    fn bounds_check(&self, address: u32, size: usize) -> ZkvmResult<usize> {
        let index = address as usize;

        if index
            .checked_add(size)
            .map(|end| end <= self.memory.len())
            .unwrap_or(false)
        {
            Ok(index)
        } else {
            Err(ZkvmError::MemoryOutOfBounds { address, size })
        }
    }
}
