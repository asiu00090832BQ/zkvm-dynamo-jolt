# zkVM Quickstart

## Installation
Ensure you have Rust and the riscv32im target installed:
```bash
rustup target add riscv32im-unknown-none-elf
```

## Building
To build the workspace:
```bash
cargo build
```
## Running Guest Program

To build a guest project:
```bash
cargo build -p hello-world-guest --target riscv32im-unknown-none-elf --release
```

To run the zkvm with the guest ELF:
```bash
cargo run --release -- target/riscv32im-unknown-none-elf/release/hello-world-guest
``a