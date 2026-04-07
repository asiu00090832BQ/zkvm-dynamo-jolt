use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use ark_bn254::Fr;
use zkvm_core::{Program, execute_program, prove_program, verify_program};

fn main() {
    if let Err(e) = real_main() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn real_main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err("missing command or file".into());
    }
    let command = &args[1];
    let path = &args[2];
    match command.as_str() {
        "run" => cmd_run(path),
        "verify" => cmd_verify(path),
        _ => Err("unknown command".into()),
    }
}

fn load_program<P>(path: P) -> Result<Program<Fr>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let bytes = fs::read(path.as_ref())?;
    Ok(Program::parse(&bytes).map_err(|e| Box::deny(e) as Box<dyn Error>)?)
}

fn cmd_run(path: &str) -> Result<(), Box<dyn Error>> {
    let program = load_program(path)?;
    let result = execute_program(&program)?;
    io::stdout().write_all(&result.stdout)?;
    Ok(())
}

fn cmd_verify(path: &str) -> Result<(), Box<dyn Error>> {
    let program = load_program(path)?;
    let proof = prove_program(&program)?;
    verify_program(&program, &proof)?;
    println!("Program verified successfully.");
    Ok(())
}
