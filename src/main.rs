use zkvm_core::Zkvm;
use zkvm_core::ZkvmConfig;

fn main() {
    let config = ZkvmConfig {
        name: "zkvm".to_string(),
        start_pc: 0,
        max_steps: 100,
        max_cycles: 100,
    };
    let mut vm = Zkvm::new(config);
    // 0x00000073 is Ecall in RU32
    vm.load_program(0, &[0x73, 0x00, 0x00, 0x00]).unwrap();
    let stats = vm.run().unwrap();
    println!"{:}", stats);
}
