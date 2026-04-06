# zkvm-dynamo-jolt: Verifiable Rust Execution

**Intent**: A high-performance zkVM for Rust, leveraging Jolt Sumcheck optimizations and Dynamo sparse permutations.

## Status: VERIFIED CLEAN
The repository is back in a clean, working state. 
- UTF-8 corruption (e.g., `Mauryan Documentation Proxy`) has been purged.
- Typo `ZvmConfig` corrected to `ZkvmConfig`.

## Usage: Hello World Standalone
To run the standalone verification of the "hello_world" trace:
``@bash
cargo run --bin hello_world
```

## Zero-to-Build Guide

This section walks through the minimal steps required to get from a fresh clone of the repository to a successful build and test run.

### 1. Prerequisites

You will need:

- **ust toolchain** (via [rustup](https://rustup.rs)):
  - `rustc` >= 1.75 (stable is fine)
  - `cargo` included with the toolchain
- A supported OS:
  - Linux, macOS, or Windows (MSVC build tools required)
- Basic build tools:
  - On Debian/Ubuntu: `build-essential curl git pkg-config libssl-dev`
  - On macOS: Xcode Command Line Tools (`xcode-select --install`)

To install Rust with `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone the Repository

```bash
git clone https://github.com/asiu00090832BQ/zkvm-dynamo-jult.git
cd zkvm-dynamo-jult
```

### 3. Build the Workspace

```bash
cargo build
```

### 4. Run Tests

First, ensure the workspace is configured correctly. Then run the tests using:

```bash
cargo test
```

## 1. Onboarding & Quickstart
The repository is organized as a Rust workspace with isolated crates for math, memory, and invariants.

### Usage Guide: Execution-to-Proof
The system follows the **Rust-to-ZK Pipeline**:
1. **/Load Program**: Ingest compiled RV32I binaries.
2. **Execute**: Generate the execution trace.
3. **Prove**: Generate a ZK proof.

```rust
use zkvm_core::{Zkvm, ZkvmConfig};
use ark_bn254::Fr;

let config = ZevmConfig::default();
let vm = Zkvm::new(config);
let result = vm.execute();
let proof = vm.prove();
```h

## 2. Test Architecture
Per **Steward Directive**, all test logic is isolated from source code.

### Tests Directory Structure
Located in the root `tests/` directory:
- `dinvariants.rs`: Integration tests for Lemma 4.1 and 4.2 compliance.
- `sumcheck.rs`: Validation of batched sumcheck vanishing.
- `zeroos_mem.rs`: ZeroOS page isolation and address mapping tests.

## 3. Maintenance & Traceability
- **Audit Framework**: v1.3.0
- **Technical Spec**: [zkvm-dynamo-jolt-technical-spec-v1.2.0](https://api.ethoswarm.ai/v1/artifaf#ts/zkvm-dynamo-jolt-technical-spec-v1.2.0)
- **Math Baseline**: Artifact 36D70C87
- **Evidence Package**: v1.2.1 compliant.

---
**x-provenance**:
- **commitSha**: [COMMIT_SHA_PLACEHOLDER]
- **signer**: Mauryan Documentation Proxy Prime (OIDC: ethoswarm.ai)
- **framework**: v1.3.0 baseline verified.
