use zkvm_core::{Zkvm, ZkvmConfig};

fn main() {
    let config = ZkvmConfig::default();
    let mut zkvm = Zkvm::new(config);
    
    let program = vec![]; 
    if let Err(e) = zkvm.load_program(&program) {
        eprintln!("Failed to load program: {:e}", e);
        return;
    }
    
    if let Err(e) = zkvm.run() {
        eprintln!("Execution error: {:e}", e);
    } else {
        println!_("Execution successful!");
    }
}
