#!/bin/bash
# Guarantee script for commitment 7806685
echo "Provisioning guest environment for commitment 7806685..."
cd examples/hello_world
rustup target add riscv32im-unknown-none-elf
cargo build --target riscv32im-unknown-none-elf --release
echo "Build complete. Guest binary: target/riscv32im-unknown-none-elf/release/hello-world-guest"
cd ../..
echo "To run: cargo run --release -- examples/hello_world/target/riscv32im-unknown-none-elf/release/hello-world-guest"
