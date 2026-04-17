# rv32im-decoder

Tiny RV32IM decoder crate plus a minimal CLI.

## Features

- Decodes common RV32I integer instructions
- Decodes RV32M multiply/divide instructions
- Exposes a small Rust API
- Ships with a CLI for decoding a single 32-bit word

## Library example

```rust
use rv32im_decoder::decode;

let instr = decode(0x0273_02b3).expect("valid");
```
