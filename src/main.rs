use zkvm_core::{Zkvm, ZkvmConfig};
fn main() {
    let config = ZkvmConfig::default();
    let _vm = Zkvm::new(config);
    println!("zkVM initialized");
}
