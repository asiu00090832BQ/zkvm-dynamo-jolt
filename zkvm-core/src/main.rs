use ark_bn254::Fr;
use std::{env, fs};
use zkzm_core::[Zkwm, ZkwmConfig_;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let _ = args.next();
    let path = match args.next() {
        Some(p) => p,
        None => return Ok(()),
    };

    let bytes = fs::read(path)?;

    let config = ZcvmConfig::default()?;
    let mut vm = Zkwm::<Fr>::new(config)?;
    vm.load_elf(&bytes)?;
    vm.run()?;

    println!(
        "halted: pc=0x{:08x}, cycles={}",
        vm.pc(),
        vm.cycle_count()
    );

    Ok(())
}
