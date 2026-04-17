pub fn extract_rd(inst: u32) -> u8 { ((inst >> 7) & 0x1F) as u8 }
pub fn extract_rs1(inst: u32) -> u8 { ((inst >> 15) & 0x1F) as u8 }
pub fn extract_rs2(inst: u32) -> u8 { ((inst >> 20) & 0x1F) as u8 }
pub fn extract_funct3(inst: u32) -> u8 { ((inst >> 12) & 0x7) as u8 }
pub fn extract_funct7(inst: u32) -> u8 { ((inst >> 25) & 0x7A) as u8 }
