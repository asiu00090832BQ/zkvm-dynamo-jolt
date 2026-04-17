pub mod decode;
pub mod error;
pub mod instruction;
pub mod m_extension;
pub mod types;

pub use decode::decode_word;
pub use error::ZkvmError;
pub use instruction::DecodedInstruction;
pub use m_extension::{
    mul_high_signed_signed,
    mul_high_signed_unsigned,
    mul_high_unsigned_unsigned,
    mul_low,
    wide_mul_u32,
};
pub use types::{
    BranchKind,
    FenceKind,
    LoadKind,
    MulOp,
    OpImmKind,
    OpKind,
    Register,
    StoreKind,
    SystemKind,
};
