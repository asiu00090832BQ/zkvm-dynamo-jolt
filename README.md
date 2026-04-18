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
## Running Arbitrary Rust File
To build a guest rust file that shares the same project root directory:
```bash
rustc --target riscv32im-unknown-none-elf -C panic=abort hello-world.rs --crate-type bin -o hello-world.elf
```

To run the zkvm with the guest program:
```bash
cargo run --release -- hello-world.elf
```

## Running Arbitrary Rust Package(No libraries)
To build a guest rust project that shares the same project root directory:

Add the guest package to the root cargo.toml workspace.

```bash
cargo build -p hello-world --target riscv32im-unknown-none-elf
```

To run the zkvm with the guest program:
```bash
cargo run --release -- hello-world.elf
```

