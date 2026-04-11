use zkvm_core::{Zkwm, ZkvmConfig};

fn main() {
    let config = ZkvmConfig {
        name: "zkvm".to_string(),
        start_pc: 0,
        max_steps: 100,
        max_cycles: 100,
    };
    let mut vm = Zkvm::new(config);
    vm.load_program(vec![0x0000_0073]);
    let stats = vm.run();
    println!("{:?}", stats);
}
