use ark_bn254::Fr;
use zkvm_core::{Zkvm, ZkvmConfig, Result};

fn main() -> Result<()> {
    let config = ZkvmConfig::default();
    let mut vm = Zkvm::<Fr>::new(config)?;
    let elf_bytes = include_bytes!("../examples/hello_world.elf");
    vm.load_elf(elf_bytes)?;
    vm.run()?;
    println!("Execution verified. Total cycles: {}", vm.cycle_count());
    Ok(())
}
