# rv32im-decoder architecture

`rv32im-decoder` is a small RV32IM decoder designed around a no-`std` core and a
`std`-gated command-line frontend.

## Module layout

- `src/decode/i.rs` decodes RV32I instructions.
- `src/decode/m.rs` decodes RV32M instructions.
- `src/decode/mod.rs` performs dispatch from the major opcode and `funct7`.
- `src/word.rs`, `src/fields.rs`, and `src/immediate.rs` provide low-level bit extraction.
- `src/compat.rs` exposes `Zkvm` and `ZkvmError` compatibility symbols.

## Limb decomposition

For implementation symmetry with proof-oriented pipelines, the instruction word can be
decomposed into two 16-bit limbs:

- low limb: bits `[15:0]`
- high limb: bits `[31:16]`

This is represented by `Limb16` and used by `Word::limbs()` / `Word::from_limbs()`.
The round-trip identity

`join(low(word), high(word)) == word`

is the operational form of the crate's Lemma 6.1.1 decomposition invariant.

## Decoding flow

1. Validate that the word is a standard 32-bit instruction (`..11` in bits `[1:0]`).
2. Extract shared fields into `Fields`.
3. Decode the major opcode.
4. Route `OP` words with `funct7 = 0b0000001` to `decode::m`.
5. Route everything else to `decode::i`.

## `std` boundary

The library core is `no_std` compatible. The CLI is available only with the `std`
feature enabled. The package enables `std` by default so `cargo build` and
`cargo run --bin rv32im-decoder -- <word>` work out of the box, while embedded users
can opt out with `--no-default-features`.
