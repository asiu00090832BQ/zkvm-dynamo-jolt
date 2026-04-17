pub mod instruction;
pub mod m_extension;

pub use instruction::{
    decode, BranchKind, CsrKind, DecodeError, ImmOpKind, Instruction, LoadKind, RegOpKind,
    Register, StoreKind,
};
pub use m_extension::{
    combine_limb16, decompose_limb16, mul_i32_u32_wide, mul_i32_wide, mul_u32_wide,
    Limb16Decomposition,
};
