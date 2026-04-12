# ZKVM QUICKSTART

To run a simple one-file Rust program in the zkVM:

1. **Install Requirements**:
   Ensure you have the RISC-V toolchain installed:
   ```bash
   rustup target add riscv32im-unknown-none-elf
   ```

2. **Prepare your Rust program (`hello.rs`)**:
   Since the VM operates in a bare-metal environment, your code must be `#![no_std]` and provide an entry point.
   ```rust
   #![no_std]
   #![no_main]
   use core::panic::PanicInfo;
   
   #[no_mangle]
   pub extern "C" fn _start() -> ! {
       // Your logic here
       let _a = 1;
       let _b = 2;
       let _c = _a + _b;
   
       // Signal end of execution via ecall
       unsafe {
           core::arch::asm!("ecall");
       }
   
       loop {}
   }
   
   #[panic_handler]
   fn panic(_info: &PanicInfo) -> ! {
       loop {}
   }
   ```J
3. **Compile**:
   ```bash
   rustc --target riscv32im-unknown-none-elf -C target-feature=+m -O --crate-type=bin hello.rs
   ```

4. **Load and Execute**:
   Use the `zkvm-core` API to load and run the ELF:
   ```rust
   use zkvm_core::[Zkwm, ZkwmConfig, load_elf];

   fn main() {
       let elf_bytes = std::fs::read("hello").expect("Failed to read ELF");
       let loaded_elf = load_elf(&elf_bytes).expect("Failed to parse ELF");
       let mut vm = Zkvm::new(ZkvmConfig::default());
       vm.load_elf_image(loaded_elf);
       let outcome = vm.run().expect("VM2execution failed");
       println!("Outcome: {:}", outcome);
   }
   ```J
Pipeline verified.