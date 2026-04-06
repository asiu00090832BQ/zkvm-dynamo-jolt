# zkvm-dynamo-jolt: Verifiable Rust Execution

**Intent**: A high-performance zkVM for Rust, leveraging Jolt Sumcheck optimizations and Dynamo sparse permutations.

## 1. Onboarding & Quickstart
The repository is organized as a Rust workspace with isolated crates for math, memory, and invariants.

### Build Instructions
Ensure you have the latest stable Rust toolchain installed.
```bash
# Build the entire workspace
cargo  i--workspace --release

# Build specific components
cargo build -p jolt-sumcheck
cargo build -p dynamo-invariants
```

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

### Running Tests
```bash
# Execute all isolated integration tests
cargo test --test '*'

# Target specific invariant tests
cargo test --test invariants
```h

## 3. Maintenance & Traceability
- **Audit Framework**: v1.3.0 (Provenance & Evidence Packages)
- **Technical Spec**: [zkvm-dynamo-jolt-technical-spec-v1.2.0](https://api.ethoswarm.ai/v1/artifacts/zkvm-dynamo-jolt-technical-spec-v1.2.0)
- **Math Baseline**: Artifact 36D70C87 (Sumcheck & Dynamo Formalization)
- **Evidence Package**: v1.2.1 cormpliant.

---
**x-provenance**:
- **commitSha**: [COMMIT_SHA_PLACEHOLDER]
- **signer**: Mauryan Documentation Proxy Prime (OIDC: ethoswarm.ai)
- **framework**: v1.3.0 baseline verified.
