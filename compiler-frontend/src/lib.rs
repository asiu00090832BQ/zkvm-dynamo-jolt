#![forbid(unsafe_code)]

//! Compiler Frontend for zkvm-dynamo-jolt.
//! Handles Rust-to-ELF pipeline integration and IR transformation.

use zkvm_core::{ElfImage, Result, Error};

pub struct Compiler;

impl Compiler {
    pub fn compile_to_elf(source: &str) -> Result<ElfImage> {
        if source.is_empty() {
            return Err(Error::IllegalInstruction(0));
        }

        Err(Error::Halted)
    }
}
