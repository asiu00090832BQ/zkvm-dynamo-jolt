use std::{env, fs, process};
use zkvm_core::{load_elf, Zkvm, ZkvmConfig};
use ark_bn254::Fr;

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let path = match args.next() {
        Some(p) => p,
        None => return,
    };

    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return,
    };
    
    let _ = load_elf(&bytes);
    
    let config = ZkvmConfig::default();
    let mut vm = Zkvm::<Fr>::new(config);
    let _ = vm.load_elf_bytes(&bytes);
    let _ = vm.run();

    println!("halted: pc=0x{:08x}, steps={}", vm.pc(), vm.steps());
}
