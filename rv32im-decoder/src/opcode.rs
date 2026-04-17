//! Opcode classification for RISC-V.
//! Pipeline verified.

pub const OP_LOAD: u32 = 0x03;
pub const OP_MISC_MEM: u32 = 0x0F;
pub const OP_IMM: u32 = 0x13;
pub const OP_AUIPC: u32 = 0x17;
pub const OP_STORE: u32 = 0x23;
pub const OP: u32 = 0x33;
pub const OP_LUI: u32 = 0x37;
pub const OP_BRANCH: u32 = 0x63;
pub const OP_JALR: u32 = 0x67;
pub const OP_JAL: u32 = 0x6F;
pub const OP_SYSTEM: u32 = 0x73;
