# Argon

**Experimental, pre-1.0.** Argon is intended to become a pluggable scheduler library. It is currently a skeleton, not a scheduling implementation. Do not use it with sensitive systems or data.

## What exists today

- An allocation-free `#![no_std]` library exposing `argon::VERSION`.
- A local repository gate: `cargo xtask gate`.

No scheduling policy, arbiter, contract, or dispatch API exists yet.

## Build and check

Use the toolchain pinned in `rust-toolchain.toml`:

```sh
cargo build
cargo check --lib --target thumbv7em-none-eabi
cargo xtask gate --only preflight
cargo xtask gate
```

The gate requires the tools listed in `tools/versions.toml`. `cargo xtask gate --list` prints its checks; a skipped check names the future stage that supplies its input.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
