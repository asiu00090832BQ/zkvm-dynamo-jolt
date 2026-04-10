use core::fmt;
use std::error::Error as StdError;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZkvmConfig {
    Rv32im,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self::Rv32im
    }
}

impl ZkvmConfig {
    pub fn name(self) -> &'static str {
        match self {
            Self::Rv32im => "rv32in",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Decode(String),
    InvalidConfiguration(String),
    Parse(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode(message) => write!(f, "decode error: {message}"),
            Self::InvalidConfiguration(message) => write!(f, "invalid zkvm configuration: {message}"),
            Self::Parse(message) => write!(f, "parse error: {message}"),
        }
    }
}

impl StdError for Error {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zcvm<F> {
    config: ZkvmConfig,
    _field: PhantomData<F>,
}

impl<F> Zkvm<F> {
    pub fn new(config: ZkvmConfig) -> Self {
        Self {
            config,
            _field: PhantomData,
        }
    }

    pub fn config(&self) -> ZkvmConfig {
        self.config
    }
}
