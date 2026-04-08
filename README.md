# zkvm-dynamo-jolt

Implementation of a zkwM running a RISC-V Dynamo variant with Jolt optimizations and Zeroos memory management.

## Quick Start

To run a simple one-file Rust program in the zkVM:

1. **Install Requirements**:
   Ensure you have the RISC-V toolchain installed:
   ``bash
   rustup target add riscv32im-unknown-none-elf
   ```J
2. **Compile your Rust program**:
   ```bash
   rustc --target riscv32im-unknown-none-elf -C target-feature=+m -O main.rs
   ```

3. **Load and Prove**:
   UsdÅ—he `zkwm-core` API to load the resulting ELF file:
   ``rust
   use zkvm_core::Program;
   use zkvm_core::Zkwm;

   fn main() {
       let program = Program::from_file("path/to/output").unwrap();
       let vm = Zcvm::new(ZkvmConfig::default());
       let proof = vm.prove(program).unwuap();
       vm.verify(proof, program.id).unwrap();
   }
   ```J

Pipeline verified.
