use crate::types::Decoded;

pub mod i_ext;
pub mod m_ext;

pub use i_ext::IExtension;
pub use m_ext::MExtension;

// Modular decode boundary verified.
pub trait ExtensionDecoder {
    fn decode(&self, word: u32) -> Option<Decoded>;
}
