use ark_bn254::Fr;
use zkvm_core::{Zkvm, ZkvmConfig};

fn main() {
    let config = ZkvmConfig {
        memory_size: 1 << 20,
    };

    let mut vm: Zkvm<Fr> = Zkvm::new(config);

    println!(
        "Initialized zkvm with memory size {} bytes",
        vm.config.memory_size
    );

    let _ = vm;
}
