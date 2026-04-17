use clap::{Parser, Subcommand};
use log::{error, info};
use std::num::ParseIntError;

use rv32im_decoder::decoder::decode;
use rv32im_decoder::m_extension::{decompose_32bit_limbs, mul_via_limbs, verify_limb_decomposition};
use rv32im_decoder::Instruction;

#[derive(Debug, Parser)]
#[command(author, version, about = "RV32IM decoder and M-extension limb verifier", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Decode { 
        instr: String 
    },
    Mul { 
        a: u32, 
        b: u32 
    },
}

fn parse_word(s: &str) -> Result<u32, ParseIntError> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16)
    } else if let Some(bin) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
        u32::from_str_radix(bin, 2)
    } else {
        u32::from_str_radix(s, 10)
    }
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Command::Decode { instr } => {
            match parse_word(&instr) {
                Ok(word) => {
                    info!("Decoding instruction word: 0x{word:08x}");
                    match decode(word) {
                        Ok(decoded) => { 
                            println!("0x{word:08x}: {decoded}"); 
                        },
                        Err(e) => {
                            error!("Decode error: {e}");
                            eprintln!("decode error: {e}");
                            std::process::exit(1);
                        }
                    }
                },
                Err(e) => {
                    error!("Failed to parse instruction word: {e}");
                    eprintln!("failed to parse instruction word: {e}");
                    std::process::exit(1);
                }
            }
        },
        Command::Mul { a, b } => {
            info!("Verifying limb decomposition for a={a}, b={b}");
            let (a0, a1, b0, b1) = decompose_32bit_limbs(a, b);
            println!("a = {a} -> a0 = {a0}, a1 = {a1}");
            println!("b = {b} -> b0 = {b0}, b1 = {b1}");

            let prod_native = (a as u64) * (b as u64);
            let prod_limbs = mul_via_limbs(a, b);

            println!("native product   = {prod_native}");
            println!("limb-based product = {prod_limbs}");

            match verify_limb_decomposition(a, b) {
                Ok(()) => { 
                    println!("lemma 6.1.1 identity holds for a and b"); 
                },
                Err(e) => {
                    error!("Limb verification failed: {e}");
                    eprintln!("limb verification failed: {e}");
                    std::process::exit(1);
                }
            }
        }
    }
}
