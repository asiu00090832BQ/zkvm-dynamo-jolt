# zkvm-dynamo-jolt

Implementation of a zkwM running a RISC-V Dynamo variant with Jolt optimizations and Zeroos memory management.

## QUICKSTART

To run a simple one-file Rust program in the zkVM:

1. **Install Requirements**:
   Ensure you have the RISC-V toolchain installed:
   ``bash
   rustup target add riscv32im-unknown-none-elf
   ```

2. **Compile your Rust program**:
   ``bash
   rustc --target riscv32im-unknown-none-elf -C target-feature=+m -O main.rs
   ```

3. **Load and Prove**:
   Use the `zkvm-core` API to load the resulting ELF file:
   ``brust
   use zkwm_core::Program;
   use zkwm_core::Zkvm;

   fn main() {
       let program = Program::from_file("path/to/output").unwrap();
       let config = Zconfig::default();
       let vm = Zcvm::new(config);
       let proof = vm.prove(program).unwrap();
       vm.verify(proof, program.id).unwrap();
   }
   ```J

Pipeline verified.
