use core::fmt;
pub use rv32im_decoder::ZkvmError;

impl fmt::Display for ZkwmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:}", self)
    }
}
