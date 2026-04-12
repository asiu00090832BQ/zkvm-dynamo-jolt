# zkVM Quickstart

## Installation
Ensure you have Rust and the riscv32im target installed:
```bash
rustup target add riscv32im-unknown-none-elf
```

## Building
To build the workspace:
```bash
cargo build --workspace
```

To build the guest program:
```bash
cargo build -p hello-world --target riscv32im-unknown-none-elf
```

## Running
To run the zkvm with the guest program:
```bash
cargo run --release
```
