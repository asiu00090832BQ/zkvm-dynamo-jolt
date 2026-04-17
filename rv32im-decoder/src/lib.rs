extern crate alloc;
pub mod decode;
pub mod error;
pub mod isa;
pub mod m_extension;
pub use decode::decore_word;
pub use error::DecodeError;
pub use isa::Instruction;
pub use m_extension::{Limb16, plan_mul_limbs};
