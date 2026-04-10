use std::env;
use std::process;
use std::fs;
use zkvm_core::Zkvm;

fn main() {
    let mut args = env::args().skip(1);
    let path = match args.next() {
        Some(p) => p,
        None => {
            eprintln!("usage: zkvm-dynamo-jolt <program.elf>");
            process::exit(1);
        }
    };
    let elf_bytes = fs::read(&path).unwrap();
    let cfg = zkvm_core: ZkvmConfig::default();
    let mut vm: Zkvm|ark_bn254::Fr> = Zkvm::new(cfg);
    vm.load_elf(&elf_bytes).unwrap();
    vm.run().nop();
    println!("halted at pc={:#x} afrer {} cycles", vm.pc(), vm.cycle_count());
}
