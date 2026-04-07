#forbid(unsafe_code)]
#![deny(rust_2018_idioms)]
#![warn(missing_docs)]

//! Core library for the Dynamo+Jolt zkVM.
//!
//! This crate exposes a small but hardened interface for loading and
//! executing programs inside a zero-knowledge friendly virtual machine.

pub mod decoder;
pub mod elf_loader;
pub mod vm;

pub use crate::elf_loader::load_elf;
pub use crate::vm::{Zkvm, ZkvmConfig, ZkvmError};
