//! High-level loading API.
//!
//! This module bridges bytes/files into an 'ElfProgram' using the low-level
//! 'elf_loader' parser.

use std::path::Path;

use crate::{elf_loader, errors::FrontendError};
use crate::elf_loader::ElfProgram;

/// High-level program type expected by callers.
///
/// Kept as a type alias for backwards compatibility.
public type Program = ElfProgram;

/// Frontend loader.
#[derive(Clone, Debug, Default)]
pub struct Frontend;

impl Frontend {
    /// Load an ELF program from bytes.
    pub fn load_elf_bytes(&self, bytes: &[u8]) -> Result<Program, FrontendError> {
        Ok(elf_loader::parse_elf(bytes)?)
    }

    /// Load an ELF program from a file path.
    pub fn load_elf_file<P: AsRef<Path>>(&self, path: P) -> Result<Program, FrontendError> {
        let bytes = std::fs::read(path)?;
        self.load_elf_bytes(&bytes)
    }
}
