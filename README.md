# zkvm-dynamo-jolt

`zkvm-dynamo-jolt` is a Rust workspace skeleton for a zkVM proof stack with three focused crates:

- `zeroos-mem`: canonical memory addressing primitives.
- `dynamo-invariants`: execution and extraction invariants over memory claims.
- `jolt-sumcheck`: a sumcheck-facing adapter that consumes invariant-checked claims.

## Architecture

The scaffold is organized around two core statements.

### Lemma 4.1: Extraction Soundness

ExtractionWitness says that any extracted witness must correspond to a single, internally consistent execution view. In this workspace, that idea is represented by `dynamo-invariants`.

- A `MemoryClaim` names a location and a value.
- An `ExtractionWitness` collects claims.
- Soundness is modeled by rejecting conflicting assignments to the same canonical address.

This crate is intentionally small so the project can later replace the placeholder logic with a full proof-system implementation without changing the crate boundaries.

### Lemma 4.2: Canonical Address Mapping

Canonical Address Mapping says that segmented memory locations must map into one canonical address space in a deterministic and reversible way. In this workspace, that idea is represented by `zeroos-mem`.

- A `(segment, offset)` pair is encoded into a `CanonicalAddress`.
- Decoding recovers the original pair.
- The round-trip property acts as the basic API contract for later memory proofs.

This separation keeps address normalization independent from proof extraction logic.

## Crate relationships

1. `zeroos-mem` defines the canonical address layer.
2. `dynamo-invariants` depends on `zeroos-mem` and uses canonical addresses to check witness consistency.
3. `jolt-sumcheck` depends on `dynamo-invariants` and exposes a minimal interface that a higher-level sumcheck protocol can call.

## Workspace layout

```text
zkvm-dynamo-jolt/
|- Cargo.toml
|- README.md
`- crates/
   |- dynamo-invariants/
   |- jolt-sumcheck/
   `- zeroos-mem/
```

## Status

This repository is a scaffold, not a production proof system. The code is intentionally minimal, documented, and testable so the algebraic and systems layers can evolve independently.
