# Reproduction of "Access violation in unknown section"

## Requirements

- Solana/Agave install set to 1.18.22
- Rust 1.75 (x86_64), code is not working with native alignment different from 64bits.

## Get it to crash

Run:

```shell
cargo test-sbf --manifest-path programs/vprog/Cargo.toml
```
