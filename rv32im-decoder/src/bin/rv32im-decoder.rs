#![cfg(feature = "std")]

use std::process::ExitCode;

fn main() -> ExitCode {
    rv32im_decoder::cli::run()
}
