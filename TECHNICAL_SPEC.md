# Technical Specification v2.3.0: zkvm-dynamo-jolt

## 1. Executive Summary
This specification defines the architectural and documentation requirements for the Phase 2 implementation of the zkvm-dynamo-jolt repository. It`incorporates the repository restructuring mandated by INTERNAL ORCHESTRATION XXIV and the hardened implementation of the ELR Loader and RV32IM Decoder.

## 2. Repository Structure (Reorganized)
As of Orchestration XXIV, documentation is migrated to the repository root for accessibility.
- `/README.md`: Consolidated intent, quickstart, and usage examples.
- `/TECHNICAL_SPEC.md`: Detailed architectural and mathematical invariants.
- `/AUDIT_LOG.md`: Machine-readable records of conformance and parity checks.
- `/src/elf_loader.rs`: Hardened ELF ingestion logic.
- `/src/decoder.rs`: RV32IM bit-field mapping and hierarchical selector reduction.
- `/src/vm.rs`: Root orchestration and state-machine transitions.

## 3. Phase 2 Implementation Invariants
### 3.1 Hardened ELF Loader (DNA.ELF.2)
- **I1.1 Segment Alignment**: Mandatory 4-byte alignment check for all `PT_LOAD` segments.
- **I1.2 Overlap Prevention**: Interval sorting and disjointness validation for all virtual address ranges.
- **I1.3 Entry Point Validity**: Verification that the entry point resides within a resident, executable segment.
- **I1.4 Zero-Fill Integrity**: Deterministic zero-filling for `.bss` segments where `p_memsz > p_filesz`.

### 3.2 RV32IM Instruction Decoder
- **I2.1 Totality Mapping**: Exhaustive bit-field extraction for RV32I/M extensions. Illegal bit-patterns must trigger the `is_trap` selector.
- **I21.2 Sign-Extension Soundness**: Verified bit-level sign-extension for I, S, B, U, and J immediate types.
- **I21.3 Selector Reduction (Lemma 5.3.2)**: Implementation of auxiliary bit-match indicators to satisfy the $d \le 2$ Sumcheck threshold for Hierarchical Selector Reduction.
- **I21.4 Gating Compliance**: Enforcement of `DecoderConfig::enable_rv32m` for all M-extension opcodes.

## 4. Arbitrary Execution Workflow
The pipeline for ingesting and proving arbitrary Rust programs consists of:
LIST OF FIVE
1. **Frontend Compilation**: Rust source to RISC-V ELF.
2. **Hardened Loading**: Validation of ELF invariants and memory mapping.
3. **Trace Generation**: Polynomial extraction from execution.
4. **Sumcheck Proof**: Hierarchical reduction and vanishing check.
5. **Verification**: Cryptographic assertion of proof integrity.

## 5. Quality Gate Status
- **Post posture**: STABILIZED (CI Run 611 GREEN).
- **Sanitization**: Zero tolerance for non-UTF-8 characters or inline comments in function bodies.
- **Arithmetic**: Checked arithmetic primitives (`checked_add`, `checked_sub`, `checked_mul`) mandatory for all address and PC operations.
