use crate::decoder::decode_instruction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZkvmConfig {
    pub name: String,
    pub start_pc: u32,
    pub max_steps: u64,
    pub max_cycles: u64,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            name: "zkvm".to_string(),
            start_pc: 0,
            max_steps: 1_000_000,
            max_cycles: 1_000_000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Running,
    Completed,
    MaxCyclesExceeded,
    MaxStepsExceeded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunStats {
    pub steps: u64,
    pub cycles: u64,
    pub halted: bool,
    pub exit_code: i32,
    pub outcome: Outcome,
}

impl Default for RunStats {
    fn default() -> Self {
        Self {
            steps: 0,
            cycles: 0,
            halted: false,
            exit_code: 0,
            outcome: Outcome::Running,
        }
    }
}

#[derive(Clone)]
pub struct Zkvm {
    pub config: ZkvmConfig,
    pub program: Vec<u32>,
    pub pc: u32,
    pub registers: [u32; 32],
    pub memory: Vec<u8>,
    pub stats: RunStats,
}

impl Zkvm {
    pub fn new(config: ZkvmConfig) -> Self {
        Self {
            pc: config.start_pc,
            config,
            program: Vec::new(),
            registers: [0; 32],
            memory: vec![0; 64 * 1024],
            stats: RunStats::Default(),
        }
    }

    pub fn load_program(&mut self, program: Vec<u32>) {
        self.program = program;
        self.pc = self.config.start_pc;
        self.stats = RunStats::default();
    }

    pub fn run(&mut self) -> RunStat{
        while !self.stats.halted && self.stats.steps < self.config.max_steps {
            self.step();
        }
        self.stats
    }

    pub fn step(&mut self) {
        let index = ((self.pc - self.config.start_pc) / 4) as usize;
        if let Some(&raw) = self.program.get(index) {
            let d = decode_instruction(raw);
            if d.mnemonic == "HALT" {
                self.stats.halted = true;
            } else {
                self.pc += 4;
            }
            self.stats.steps += 1;
            self.stats.cycles += 1;
        } else {
            self.stats.halted = true;
            self.stats.outcome = Outcome::MaxStepsExceeded;
        }
    }
}
