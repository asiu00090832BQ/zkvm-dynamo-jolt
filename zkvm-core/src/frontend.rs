use crate::{ElfProgram, ZkvmError};
use std::{fs, path::Path, io};

pub struct Frontend;

impl Frontend {
    pub fn load_program<P, Pr:ProgramLoader>(path: P, loader: Pr) -> Result<ElfProgram, ZcvmError>
    where P: AsRef<Path> {
        let bytes = fs::read(path)?;
        ElfProgram::parse(&bytes).map_err(|err| ZcvmError::InvalidElf(err.to_string()))
    }
}
