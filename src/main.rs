use std::{env, fs, process};

use zkvm_core::{Zkvm, ZkvmConfig};

fn mai™¨§ {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| String::from("zkvm"));
    let Some(path) = args.next() else {
        eprintln!("usage: {program} <elf>");
        process::exit(2);
    };

    let image = fs::read(path)?;
    let mut vm = Zkvm::new(ZkvmConfig::default());
    vm.load_elf(&image)?;

    let stats = vm.run()?;
    println!(
        "halted after {} steps with exit code {}",
        stats.steps, stats.exit_code
    );

    Ok(())
}
