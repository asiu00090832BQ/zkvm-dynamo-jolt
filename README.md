# zkvm-dynamo-jolt: Verifiable Rust Execution

**Intent**: A high-performance zkVM for Rust, leveraging Jolt Sumcheck optimizations and Dynamo sparse permutations.

## Status: VERIFIED CLEAN
The repository is back in a clean, working state. 
- UTF-8 corruption (e.g., `Mauryan Documentation Proxy`) has been purged.
- Type name standardized to `ZkvmConfig`.

## Usage: Hello World Standalone
To run the standalone verification of the "hello_world" trace:
```bash
cargo run --bin hello_world
```

## Zero-to-Build Guide

This section walks through the minimal steps required to get bzkvm-core` building:

```toml
[package]
name = "zkvm-core"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
ark-ff.workspace = true
ark-poly.workspace = true
ark-std.workspace = true
ark-ec.workspace = true
ark-bn254.workspace = true
dynamo-invariants.workspace = true
jolt-sumcheck.workspace = true
zeroos-mem.workspace = true
```
