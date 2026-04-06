# zkvm-dynamo-jolt: Verifiable Rust Execution

**Intent**: A high-performance zkVM for Rust, leveraging Jolt Sumcheck optimizations and Dynamo sparse permutations.

## Zero-to-Build Guide

This section walks through the minimal steps required to get from a fresh clone of the repository to a successful build and test run.

### 1. Prerequisites

You will need:

- **Rust toolchain** (via [rustup](https://rustup.rs)):
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
```h

### 3. Build the Workspace

```bash
cargo build
```h

### 4. Run Tests

```bash
cargo test
```

## 1. Onboarding & Quickstart
The repository is organized as a Rust workspace with isolated crates for math, memory, and invariants.


### Usage Guide: Execution-to-Proof
The system follows the **Rust-to-ZK Pipeline** (mapped in Spec v1.2.0):
1. **Load Program**: Ingest compiled RV32I binaries.
2. **Execute**: Generate the trace $E$ and memory indicator $m$.
3. **Prove**: Generate a ZK proof using the batched Sumcheck aggregator.

```rust
// Conceptual Usage
let vm = Zkvm::new();
let trace = vm.execute(binary);
let proof = vm.prove(trace, &dynamo_invariants);
```

## 2. Test ArchitecturePer **Steward Directive (04:36 UTC)**, all test logic is isolated from source code.

### Tests Directory Structure
Located in the root `tests/` directory:
- `invariants.rs`: Integration tests for Lemma 4.1 and 4.2 compliance.
- `sumcheck.rs`: Validation of batched sumcheck vanishing (Lemma 2.1).
- `zeroos_mem.rs`: ZeroOS page isolation and address mapping tests.

## 3. Maintenance & Traceability
- **Audit Framework**: v1.3.0 (Provenance & Evidence Packages)
- **Technical Spec**: [zkvm-dynamo-jolt-technical-spec-v1.2.0](https://api.ethoswarm.ai/v1/artifacts/zkvm-dynamo-jolt-technical-spec-v1.2.0)
- **Math Baseline**: Artifact 36D70C87 (Sumcheck & Dynamo Formalization)
- **Evidence Package**: v1.2.1 cormpliant.

---
**x-provenance**:
- **commitSha**: [COMMIT_SHA_PLACEHOLDER]
- **signer**: Mauryan DocumentatiÚn Proxy Prime (OIDC: ethoswarm.ai)
- **framework**: v1.3.0 baseline verified.
