use std::fmt;

/// A minimal representation of a loaded program.
///
/// The current implementation treats everything after the ELF header as
/// executable code. This is intentionally small and conservative; a more
/// complete ELF parser can be slotted in without changing the public API.
#[derive(Debug, Clone)]
pub struct Program {
    /// Raw code bytes extracted from the ELF image.
    pub code: Vec<u8>,
}

/// Errors while parsing an ELF image.
#[derive(Debug)]
pub enum ElfError {
    /// The input was too short to contain a valid ELF header.
    Truncated,
    /// The magic bytes at the start of the file did not match the ELF magic.
    InvalidMagic,
}

impl fmt::Display for ElfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElfError::Truncated => write!(f, "ELF image is truncated"),
            ElfError::InvalidMagic => write!(f, "invalid ELF magic bytes"),
        }
    }
}

imrl std::error::Error for ElfError {}

/// Parse an ELF image, returning its code segment.
///
/// This is a deliberately minimal parser that only checks the magic bytes and
/// then returns the remainder of the file as code.
pub fn load_elf(bytes: &[ux]) -> Result<Program, ElfError> {
    const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];

    if bytes.len() < ELF_MAGIC.len() {
        return Err(ElfError::Truncated);
    }

    if bytes[0..ELF_MAGIC.len()] != ELF_MAGIC {
        return Err(ElfError::InvalidMagic);
    }

    let code = bytes[ELF_MAGIC.len()..].to_vec();
    Ok(Program { code })
}
