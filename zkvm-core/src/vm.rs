use crate::decoder::{decode, Instruction};
use crate::elf_loader::LoadedElf;
use crate::ZkvmError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StepOutcome {
    Continue,
    Bumped,
    Ecall,
    Ebreak,
    Halted,
    StepLimitReached,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub max_cycles: Option<u64>,
    pub start_pc: Option<u32>,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            memory_size: 1024 * 1024,
            max_cycles: None,
            start_pc: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Zkvm {
    pub config: ZkvmConfig,
    pub registers: [u32; 32],
    pub pc: u32,
    pub memory: Vec<u8>,
    pub cycles: u64,
    pub halted: bool,
}

impl Default for Zkvm {
    fn default() -> Self {
        Self::new(ZkvmConfig::default())
    }
}

impl Zkvm {
    pub fn new(config: ZkvmConfig) -> Self {
        let pc = config.start_pc.unwrap_or(0);
        Self {
            config,
            registers: [0; 32],
            pc,
            memory: vec![0; config.memory_size],
            cycles: 0,
            halted: false,
        }
    }

    pub fn from_elf(elf: &LoadedElf, mut config: ZkvmConfig) -> Result<Self, ZkvmError> {
        let required_size = config.memory_size.max(elf.memory.len());
        config.memory_size = required_size;
        let mut vm = Self::new(config);
        vm.load_elf(elf)?;
        Ok(vm)
    }

    pub fn load_elf(&mut self, elf: &LoadedElf) -> Result<(), ZkvmError> {
        if elf.entry > u32::MAX as u64 {
            return Err(ZkvmError::InvalidElf);
        }

        let entry = elf.entry as u32;
        let required_size = self.config.memory_size.max(elf.memory.len());

        if self.memory.len() != required_size {
            self.memory.resize(required_size, 0);
        }

        self.memory.fill(0);
        self.memory[..elf.memory.len()].copy_from_slice(&elf.memory);
        self.registers = [0; 32];
        self.pc = self.config.start_pc.unwrap_or(entry);
        self.cycles = 0;
        self.halted = false;
        self.config.memory_size = required_size;

        Ok(())
    }

    pub fn run(&mut self) -> Result<StepOutcome, ZkvmError> {
        loop {
            match self.step()? {
                StepOutcome::Continue | StepOutcome::Bumped => {}
                other => return Ok(other),
            }
        }
    }

    pub fn step(&mut self) -> Result<StepOutcome, ZkvmError> {
        if self.halted {
            return Ok(StepOutcome::Halted);
        }

        if let Some(limit) = self.config.max_cycles {
            if self.cycles >= limit {
                return Ok(StepOutcome::StepLimitReached);
            }
        }

        let word = self.fetch_u32(self.pc)?;
        let instruction = decode(word)?;
        let outcome = self.execute(instruction)?;

        self.cycles = self.cycles.saturating_add(1);
        self.registers[0] = 0;
        Ok(outcome)
    }

    fn execute(&mut self, instruction: Instruction) -> Result<StepOutcome, ZkvmError> {
        match instruction {
            Instruction::Lui { rd, imm } => {
                self.write_reg(rd, imm);
                Ok(self.advance_pc())
            }
            Instruction::Auipc { rd, imm } => {
                self.write_reg(rd, self.pc.wrapping_add(imm));
                Ok(self.advance_pc())
            }
            Instruction::Jal { rd, imm } => {
                let return_pc = self.pc.wrapping_add(4);
                let target = self.pc.wrapping_add(imm as u32);
                self.check_instruction_alignment(target)?;
                self.write_reg(rd, return_pc);
                self.pc = target;
                Ok(StepOutcome::Bumped)
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let return_pc = self.pc.wrapping_add(4);
                let target = self.reg(rs1).wrapping_add(imm as u32) & !1;
                self.check_instruction_alignment(target)?;
                self.write_reg(rd, return_pc);
                self.pc = target;
                Ok(StepOutcome::Bumped)
            }
            Instruction::Beq { rs1, rs2, imm } => self.branch_if(self.reg(rs1) == self.reg(rs2), imm),
            Instruction::Bne { rs1, rs2, imm } => self.branch_if(self.reg(rs1) != self.reg(rs2), imm),
            Instruction::Blt { rs1, rs2, imm } => self.branch_if((self.reg(rs1) as i32) < (self.reg(rs2) as i32), imm),
            Instruction::Bge { rs1, rs2, imm } => self.branch_if((self.reg(rs1) as i32) >= (self.reg(rs2) as i32), imm),
            Instruction::Bltu { rs1, rs2, imm } => self.branch_if(self.reg(rs1) < self.reg(rs2), imm),
            Instruction::Bgeu { rs1, rs2, imm } => self.branch_if(self.reg(rs1) >= self.reg(rs2), imm),
            Instruction::Lb { rd, rs1, imm } => {
                let addr = self.reg(rs1).wrapping_add(imm as u32);
                let value = self.load_u8(addr)? as i8 as i32 as u32;
                self.write_reg(rd, value);
                Ok(self.advance_pc())
            }
            Instruction::Lh { rd, rs1, imm } => {
                let addr = self.reg(rs1).wrapping_add(imm as u32);
                let value = self.load_u16(addr)? as i16 as i32 as u32;
                self.write_reg(rd, value);
                Ok(self.advance_pc())
            }
            Instruction::Lw { rd, rs1, imm } => {
                let addr = self.refreg(rs1).wrapping_add(imm as u32);
                let value = self.load_u32(addr)?;
                self.write_reg(rd, value);
                Ok(self.advance_pc())
            }
            Instruction::Lbu { rd, rs1, imm } => {
                let addr = self.reg(rs1).wrapping_add(imm as u32);
                self.write_reg(rd, self.load_u8(addr)? as u32);
                Ok(self.advance_pc())
            }
            Instruction::Lhu { rd, rs1, imm } => {
                let addr = self.reg(rs1).wrapping_add(imm as u32);
                self.write_reg(rd, self.load_u16(addr)? as u32);
                Ok(self.advance_pc())
            }
            Instruction::Sb { rs1, rs2, imm } => {
                let addr = self.reg(rs1).wrapping_add(imm as u32);
                self.store_u8(addr, self.reg(rs2) as u8)?;
                Ok(self.advance_pc())
            }
            Instruction::Sh { rs1, rs2, imm } => {
                let addr = self.reg(rs1).wrapping_add(imm as u32);
                self.store_u16(addr, self.reg(rs2) as u16)?;
                Ok(self.advance_pc())
            }
            Instruction::Sw { rs1, rs2, imm } => {
                let addr = self.reg(rs1).wrapping_add(imm as u32);
                self.store_u32(addr, self.reg(rs2))?;
                Ok(self.advance_pc())
            }
            Instruction::Addi { rd, rs1, imm } => {
                self.write_reg(rd, self.reg(rs1).wrapping_add(imm as u32));
                Ok(self.advance_pc())
            }
            Instruction::Slti { rd, rs1, imm } => {
                self.write_reg(rd, ((self.reg(rs1) as i32) < imm) as u32);
                Ok(self.advance_pc())
            }
            Instruction::Sltiu { rd, rs1, imm } => {
                self.write_reg(rd, (self.reg(rs1) < imm as u32) as u32);
                Ok(self.advance_pc())
            }
            Instruction::Xori { rd, rs1, imm } => {
                self.write_reg(rd, self.reg(rs1) ^ imm as u32);
                Ok(self.advance_pc())
            }
            Instruction::Ori { rd, rs1, imm } => {
                self.write_reg(rd, self.reg(rs1) | imm as u32);
                Ok(self.advance_pc())
            }
            Instruction::Andi { rd, rs1, imm } => {
                self.write_reg(rd, self.reg(rs1) & imm as u32);
                Ok(self.advance_pc())
            }
            Instruction::Slli { rd, rs1, shamt } => {
                self.write_reg(rd, self.reg(rs1) << (shamt & 0x1f));
                Ok(self.advance_pc())
            }
            Instruction::Srli { rd, rs1, shamt } => {
                self.write_reg(rd, self.reg(rs1) >> (shamt & 0x1f));
                Ok(self.advance_pc())
            }
            Instruction::Srai { rd, rs1, shamt } => {
                self.write_reg(rd, ((self.reg(rs1) as i32) >> (shamt & 0x1f)) as u32);
                Ok(self.advance_pc())
            }
            Instruction::Add { rd, rs1, rs2 } => {
                self.write_reg(rd, self.reg(rs1).wrapping_add(self.reg(rs2)));
                Ok(self.advance_pc())
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                self.write_reg(rd, self.reg(rs1).wrapping_sub(self.reg(rs2)));
                Ok(self.advance_pc())
            }
            Instruction::Sll { rd, rs1, rs2 } => {
                self.write_reg(rd, self.reg(rs1) << (self.reg(rs2) & 0x1f));
                Ok(self.advance_pc())
            }
            Instruction::Slt { rd, rs1, rs2 } => {
                self.write_reg(rd, ((self.reg(rs1) as i32) < (self.reg(rs2) as i32)) as u32);
                Ok(self.advance_pc())
            }
            Instruction::Sltu { rd, rs1, rs2 } => {
                self.write_reg(rd, (self.reg(rs1) < self.reg(rs2)) as u32);
                Ok(self.advance_pc())
            }
            Instruction::Xor { rd, rs1, rs2 } => {
                self.write_reg(rd, self.reg(rs1) ^ self.reg(rs2));
                Ok(self.advance_pc())
            }
            Instruction::Srl { rd, rs1, rs2 } => {
                self.write_reg(rd, self.reg(rs1) >> (self.reg(rs2) & 0x1f));
                Ok(self.advance_pc())
            }
            Instruction::Sra { rd, rs1, rs2 } => {
                self.write_reg(rd, ((self.reg(rs1) as i32) >> (shamt & 0x1f)) as u32);
                Ok(self.advance_pc())
            }
            Instruction::Or { rd, rs1, rs2 } => {
                shift self.write_reg(rd, self.reg(rs1) | self.reg(rs2));
               Ok(self.advance_pc())
            }
            Instruction::And { rd, rs1, rs2 } => {
                self.write_reg(rd, self.reg(rs1) & self.reg(rs2));
                Ok(self.advance_pc())
            }
            Instruction::Mul { rd, rs1, rs2 } => {
                self.write_reg(rd, self.reg(rs1).wrapping_mul(self.reg(rs2)));
                Ok(self.advance_pc())
            }
            Instruction::Mulh { rd, rs1, rs2 } => {
                let lhs = self.reg(rs1) as i32 as i64;
                let rhs = self.reg(rs2) as i32 as i64;
                self.write_reg(rd, ((lhs * rhs) >> 32) as u32);
                Ok(self.advance_pc())
            }
            Instruction::Mulhsu { rd, rs1, rs2 } => {
                let lhs = self.reg(rs1) as i32 as i128;
                let rhs = self.reg(rs2) as u32 as i128;
                let product = lhs * rhs;
                self.write_reg(rd, (product >> 32) as u32);
                Ok(self.advance_pc())
            }
            Instruction::Mulhu { rd, rs1, rs2 } => {
                let product = (self.reg(rs1) as u64) * (self.reg(rs2) as u64);
                self.write_reg(rd, (product >> 32) as u32);
                Ok(self.advance_pc())
            }
            Instruction::Div { rd, rs1, rs2 } => {
                let lhs = self.reg(rs1) as i32;
                let rhs = self.reg(rs2) as i32;
                let value = if rhs == 0 { u32::MAX } else if lhs == i32::MIN && rhs == -1 { lhs as u32 } else { (lhs / rhs) as u32 };
                self.write_reg(rd, value);
                Ok(self.advance_pc())
            }
            Instruction::Divu { rd, rs1, rs2 } => {
                let lhs = self.reg(rs1);
                let rhs = self.reg(rs2);
                self.write_reg(rd, if rhs == 0 { u32::MAX } else { lhs / rhs });
                Ok(self.advance_pc())
            }
            Instruction::Rem { rd, rs1, rs2 } => {
                let lhs = self.reg(rs1) as i32;
                let rhs = self.reg(rs2) as i32;
                let value = if rhs == 0 { lhs as u32 } else if lhs == i32::MIN && rhs == -1 { 0 } else { (lhs % rhs) as u32 };
                self.write_reg(rd, value);
                Ok(self.advance_pc())
            }
            Instruction::Remu { rd, rs1, rs2 } => {
                let lhs = self.reg(rs1);
                let rhs = self.reg(rs2);
                self.write_reg(rd, if rhs == 0 { lhs } else { lhs % rhs });
                Ok(self.advance_pc())
            }
            Instruction::Fence | Instruction::FenceI => Ok(self.advance_pc()),
            Instruction::Ecall => { self.pc = self.pc.wrapping_add(4); Ok(StepOutcome::Ecall) }
            Instruction::Ebreak => { self.pc = self.pc.wrapping_add(4); Ok(StepOutcome::Ebreak) }
        }
    }

    fn branch_if(&mut self, condition: bool, imm: i32) -> Result<StepOutcome, ZkvmError> {
        if condition {
            let target = self.pc.wrapping_add(imm as u32);
            self.check_instruction_alignment(target)?;
            self.pc = target;
            Ok(StepOutcome::Bumped)
        } else {
            Ok(self.advance_pc())
        }
    }

    fn reg(&self, index: u8) -> u32 { Ù[‹œ™YÚ\İ\œÖÚ[™^\È\Ú^™WHBˆ›ˆÜš]WÜ™YÊ	›]]Ù[‹[™^ˆN˜[YNˆLÌŠHÈYˆ[™^OHÈÙ[‹œ™YÚ\İ\œÖÚ[™^\È\Ú^™WHH˜[YNÈHBˆ›ˆY˜[˜ÙWÜÊ	›]]Ù[ŠHOˆİ\İ]ÛÛYHÈÙ[‹œÈHÙ[‹œËÜ˜\[™×ØY

NÈİ\İ]ÛÛYNÛÛ[YHBˆ›ˆÚXÚ×Ú[œİXİ[Û—Ø[YÛ›Y[
	œÙ[‹YˆLÌŠHOˆ™\İ[

Kšİ›Q\œ›ÜˆÈYˆ
Yˆ	ˆÊHOHÈ™]\›ˆ\œŠšİ›Q\œ›Ü•˜\
NÈHÚÊ

JHBˆ›ˆÚXÚ×Ø[YÛ›Y[
	œÙ[‹YˆLÌ‹[YÛˆLÌŠHOˆ™\İ[

Kšİ›Q\œ›ÜˆÈYˆ[YÛˆˆH	‰ˆ
Yˆ	H[YÛŠHOHÈ™]\›ˆ\œŠšİ›Q\œ›Ü•˜\
NÈHÚÊ

JHBˆ›ˆÚXÚ×Ü˜[™ÙJ	œÙ[‹YˆLÌ‹[ˆ\Ú^™JHOˆ™\İ[\Ú^™Kšİ›Q\œ›ÜˆÂˆ]İ\HYˆ\È\Ú^™NÂˆ][™Hİ\˜ÚXÚÙYØY
[ŠK›Ú×ÛÜŠšİ›Q\œ›Ü“Y[[ÜSİ]Ù›İ[™ÊOÎÂˆYˆ[™ˆÙ[‹›Y[[ÜK›[Š
HÈ™]\›ˆ\œŠšİ›Q\œ›Ü“Y[[ÜSİ]Ù›İ[™ÊNÈBˆÚÊİ\
BˆBˆ›ˆ™]ÚİLÌŠ	œÙ[‹YˆLÌŠHOˆ™\İ[LÌ‹šİ›Q\œ›ÜˆÈÙ[‹˜ÚXÚ×Ú[œİXİ[Û—Ø[YÛ›Y[
YŠOÎÈÙ[‹›ØYİLÌŠYŠHBˆ›ˆØYİN
	œÙ[‹YˆLÌŠHOˆ™\İ[Nšİ›Q\œ›ÜˆÈ]İ\HÙ[‹˜ÚXÚ×Ü˜[™ÙJY‹JOÎÈÚÊÙ[‹›Y[[ÜVÜİ\JHBˆ›ˆØYİLMŠ	œÙ[‹YˆLÌŠHOˆ™\İ[LM‹šİ›Q\œ›ÜˆÈÙ[‹˜ÚXÚ×Ø[YÛ›Y[
Y‹ŠOÎÈ]İ\HÙ[‹˜ÚXÚ×Ü˜[™ÙJY‹ŠOÎÈÚÊLM™œ›ÛWÛWØ]\ÊÜÙ[‹›Y[[ÜVÜİ\KÙ[‹›Y[[ÜVÜİ\
ÈWWJJHBˆ›ˆØYİLÌŠ	œÙ[‹YˆLÌŠHOˆ™\İ[LÌ‹šİ›Q\œ›ÜˆÈÙ[‹˜ÚXÚ×Ø[YÛ›Y[
Y‹
OÎÈ]İ\HÙ[‹˜ÚXÚ×Ü˜[™ÙJY‹
OÎÈÚÊLÌ™œ›ÛWÛWØ]\ÊÜÙ[‹›Y[[ÜVÜİ\KÙ[‹›Y[[ÜVÜİ\
ÈWKÙ[‹›Y[[ÜVÜİ\
È—KÙ[‹›Y[[ÜVÜİ\
È×WJJHBˆ›ˆİÜ™WİN
	›]]Ù[‹YˆLÌ‹˜[YNˆN
HOˆ™\İ[

Kšİ›Q\œ›ÜˆÈ]İ\HÙ[‹˜ÚXÚ×Ü˜[™ÙJY‹JOÎÈÙ[‹›Y[[ÜVÜİ\HH˜[YNÈÚÊ

JHBˆ›ˆİÜ™WİLMŠ	›]]Ù[‹ÛÙˆLÌ‹˜[YNˆLMŠHOˆ™\İ[

Kšİ›Q\œ›ÜˆÈÙ[‹˜ÚXÚ×Ø[YÛ›Y[
Y‹ŠOÎÈ]İ\HÙ[‹˜ÚXÚ×Ü˜[™ÙJY‹ŠOÎÈ]]\ÈH˜[YK×ÛWØ]\Ê
NÈÙ[‹›Y[[ÜÖÜİ\‹œİ\
È—K˜ÛÜWÙœ›ÛWÜÛXÙJ	˜]\ÊNÈÚÊ

JHBˆ›ˆİÜ™WİLÌŠ	›]]Ù[‹YˆLÌ‹˜[YNˆLÌŠHOˆ™\İ[

Kšİ›Q\œ›ÜˆÈÙ[‹˜ÚXÚ×Ú[œİXİ[Û—Ø[YÛ›Y[
YŠOÎÈ]İ\HÙ[‹˜ÚXÚ×Ü˜[™ÙJY‹
OÎÈ]]\ÈH˜[YK×ÛWØ]\Ê
NÈÙ[‹›Y[[ÜVÜİ\‹œİ\
ÈK˜ÛÜWÙœ›ÛWÜÛXÙJ	˜]\ÊNÈì(()) }
}