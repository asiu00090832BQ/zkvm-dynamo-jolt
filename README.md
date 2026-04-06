# zkvm-dynamo-jolt

[![Build](https://github.com/asiu00090832BQ/zkvm-dynamo-jolt/actions/workflows/build.yml/badge.svg)](https://github.com/asiu00090832BQ/zkvm-dynamo-jolt/actions/workflows/build.yml)
[![Test](https://github.com/asiu00090832BQ/zkvm-dynamo-jolt/actions/workflows/test.yml/badge.svg)](https://github.com/asiu00090832BQ/zkvm-dynamo-jolt/actions/workflows/test.yml)
[![Lint](https://github.com/asiu00090832BQ/zkvm-dynamo-jolt/actions/workflows/lint.yml/badge.svg)](https://github.com/asiu00090832BQ/zkvm-dynamo-jolt/actions/workflows/lint.yml)

High-integrity zkVM proof stack using Jolt and Dynamo invariants.

## Workspace Layout

- `dynamo-invariants/` — invariant definitions and constraint plumbing.
- `jolt-sumcheck/` — sumcheck components for proof generation.
- `zeroos-mem/` — memory-model utilities for zkVM execution traces.
- `zkvm-core/` — shared core interfaces, proving orchestration, and verifier-facing types.

## Build

```bash
cargo build
cargo test
```

## Usage

Conceptual Rust-to-ZK proof flow:

1. Implement the Rust program or circuit-facing logic in `zkvm-core`.
2. Model memory transitions and execution state with `zeroos-mem`.
3. Encode execution constraints and Dynamo invariants in `dynamo-invariants`.
4. Run prover-side sumcheck and aggregation logic through `jolt-sumcheck`.
5. Produce a proof artifact and verify it with the workspace verifier path.
