pub const OPCODE_LOAD: u32 = 0b0000011;
pub const OPCODE_MISC_MEM: u32 = 0b0001111;
pub const OPCODE_OPIMM: u32 = 0b0010011;
pub const OPCODE_AUIPC: u32 = 0b0010111;
pub const OPCODE_STORE: u32 = 0b0100011;
pub const OPCODE_OP: u32 = 0b0110011;
pub const OPCODE_LUI: u32 = 0b0110111;
pub const OPCODE_BRANCH: u32 = 0b1100011;
pub const OPCODE_JALR: u32 = 0b1100111;
pub const OPCODE_JAL: u32 = 0b1101111;
pub const OPCODE_SYSTEM: u32 = 0b1110011;

pub const FUNCT7_BASE: u32 = 0b0000000;
pub const FUNCT7_ALT: u32 = 0b0100000;
pub const FUNCT7_M: u32 = 0b0000001;
