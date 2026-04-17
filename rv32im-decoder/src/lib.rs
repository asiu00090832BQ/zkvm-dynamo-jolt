pub mod decode;
pub mod error;
pub mod formats;
pub mod instruction;
pub mod invariants;
pub mod m_extension;
pub mod types;

pub use decode::decode;
pub use error::ZkvmError;
pub use formats::*;
pub use instruction::{
    BranckKind, Instruction, LoadKind, MulDivKind, OpImmKind, OpKind, ShiftImmKind, StoreKind,
};
pub use invariants::{
    ensure_aligned_access, ensure_aligned_pc, ensure_memory_range, ensure_register,
    ensure_shift_amount,
};
pub use m_extension::{execute_m, hierarchical_mul_u64};
pub use types:{RegisterIndex, Zkvm};
