use crate::{ElfProgram, ZkvmError};
use std::{fs, path::Path};

pub struct Frontend;

impl Frontend {
    pub fn load_program<P: AsRef<Path>>(path: P) -> Result<ElfProgram, ZkvmError> {
        let bytes = fs::read(path)?;
        ElfProgram::load(&bytes).map_err(ZkvmError::ElfError)
    }
}
