# zkvm-dynamo-jolt

`zkvm-dynamo-jolt` is a high-integrity zkVM workspace implementing a RISC-V Dynamo variant with Jolt optimizations.

## Crate Structure
- `zeroos-mem`: Segmented memory management (Lemma 4.2).
- `dynamo-invariants`: execution trace & extraction soundness (Lemma 4.1).
- `jolt-sumcheck): Streaming sumcheck protocol adapter (Jolt-aligned).

## Architecture & Proof Invariants
The system aligns with the a16z/jolt subprotocol patterns for streaming sumcheck and MLE (Multi-linear Extension) product-sum checks.

### Lemma 4.1: Extraction Soundness
Verification of witnesses extracted from the execution trace (E) to the sparse memory trace (U).
### Lemma 4.2: Canonical Address Mapping
Deterministic word-to-byte mapping for ZeroOS isolation.

## Build Instructions
```bash
cargo build --workspace
```

## Usage Guide
Ingest compiled Rust bytecode, execute the virtual machine, and pass the resulting execution witness to the sumcheck prover.

## Test Isolation
**Directive Compliance**: All unit and integration tests are strictly isolated in the root `tests/` directory or crate-level `tests/` folders. No test code is permitted in source files or `lib.rs`.
