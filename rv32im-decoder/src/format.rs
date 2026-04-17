use crate::opcode::Opcode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    R,
    I,
    S,
    B,
    U,
    J,
    System,
}

impl Format {
    pub const fn from_opcode(opcode: Opcode) -> Self {
        match opcode {
            Opcode::Op => Self::R,
            Opcode::Load | Opcode::OpImm | Opcode::Jalr | Opcode::MiscMem => Self::I,
            Opcode::Store => Self::S,
            Opcode::Branch => Self::B,
            Opcode::Lui | Opcode::Auipc => Self::U,
            Opcode::Jal => Self::J,
            Opcode::System => Self::System,
        }
    }
}
