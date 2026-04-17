use crate::isa::RTypeFields;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rv32M {
    Mul(RTypeFields),
    Mulh(RTypeFields),
    Mulhsu(RTypeFields),
    Mulhu(RTypeFields),
    Div(RTypeFields),
    Divu(RTypeFields),
    Rem(RTypeFields),
    Remu(RTypeFields),
}

impl Rv32M {
    pub fn fields(self) -> RTypeFields {
        match self {
            Self::Mul(fields) => fields,
            Self::Mulh(fields) => fields,
            Self::Mulhsu(fields) => fields,
            Self::Mulhu(fields) => fields,
            Self::Div(fields) => fields,
            Self::Divu(fields) => fields,
            Self::Rem(fields) => fields,
            Self::Remu(fields) => fields,
        }
    }
}
