use ark_ff::PrimeField;
use core::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trap {
    Halt,
    InvalidOpcode(u8),
    StepLimitExceeded,
    ProgramOutOfBounds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Halt,
    Nop,
}

impl Instruction {
    pub fn decode(byte: u8) -> Result<Self, Trap> {
        match byte {
            0x00 => Ok(Instruction::Halt),
            0x01 => Ok(Instruction::Nop),
            other => Err(Trap::InvalidOpcode(other)),
        }
    }
}

pub struct Vm<F: PrimeField> {
    pc: usize,
    memory: Vec<u8>,
    step_limit: usize,
    steps: usize,
    _field: PhantomData<F>,
}

impl<F: PrimeField> Vm<F> {
    pub fn new(memory: Vec<u8>, step_limit: usize) -> Self {
        Vm {
            pc: 0,
            memory,
            step_limit,
            steps: 0,
            _field: PhantomData,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.memory.clear();
        self.memory.extend_from_slice(program);
        self.pc = 0;
        self.steps = 0;
    }

    pub fn step(&mut self) -> Result<(), Trap> {
        if self.steps >= self.step_limit {
            return Err(Trap::StepLimitExceeded);
        }
        if self.pc >= self.memory.len() {
            return Err(Trap::ProgramOutOfBounds);
        }
        let opcode = self.memory[self.pc];
        let instr = Instruction::decode(opcode)?;
        self.pc += 1;
        self.steps += 1;
        match instr {
            Instruction::Halt => Err(Trap::Halt),
            Instruction::Nop => Ok(()),
        }
    }

    pub fn run(&mut self) -> Result<(), Trap> {
        loop {
            match self.step() {
                Ok(()) => {}
                Err(Trap::Halt) => return Ok(()),
                Err(e) => return Err(e),
            }
        }
    }
}
