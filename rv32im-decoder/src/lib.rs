pub mod decoder;
pub mod i_extension;
pub mod m_extension;
pub mod types;
pub mod util;

pub use decoder::decode;
pub use m_extension::{
    div_i32, div_u32, mul_u32_low, mulh_i32_i32, mulhsu_i32_u32, mulhu_u32_u32, rem_i32, rem_u32,
};
pub use types::{DecodeError, Instruction};
