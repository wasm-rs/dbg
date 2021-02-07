## dbg! for wasm32
[![Crate](https://img.shields.io/crates/v/wasm-rs-dbg.svg)](https://crates.io/crates/wasm-rs-dbg)
[![API](https://docs.rs/wasm-rs-dbg/badge.svg)](https://docs.rs/wasm-rs-dbg)
[![Chat](https://img.shields.io/discord/807386653852565545.svg?logo=discord)](https://discord.gg/qbcbjHWjaD)

This micro-crate provides a drop-in replacement for [`std::dbg`](https://doc.rust-lang.org/std/macro.dbg.html) macro
that logs to `console` when compiled for `wasm32` and falls back to `std::dbg` on all other platforms.

## Usage

Include this dependency in your `Cargo.toml`:

```toml
[dependencies]
wasm-rs-dbg = "0.1.0"
```

Then, in files where you want to use WebAssembly-enabled logging to `console`, use this import:

```rust
use wasm_rs_dbg::dbg;
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
