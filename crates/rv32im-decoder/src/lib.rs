#![cfg_attr(not(feature = "std"), no_std)]
pub mod decoder;
pub mod error;
pub mod types;
pub use decoder::decode;
pub use error::{ZkvmError, ZkvmResult};
pub use types::*;
