# Bytes

A utility library for working with bytes. This is fork of bytes crate (https://github.com/tokio-rs/bytes)

[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/loony-bytes.svg
[crates-url]: https://crates.io/crates/loony-bytes

[Documentation](https://docs.rs/loony-bytes)

## Usage

To use `bytes`, first add this to your `Cargo.toml`:

```toml
[dependencies]
loony-bytes = { git="https://github.com/sankar-boro/loony" }
```

Next, add this to your crate:

```rust
use loony_bytes::{Bytes, BytesMut, Buf, BufMut};
```

## Serde support

Serde support is optional and disabled by default. To enable use the feature `serde`.

```toml
[dependencies]
loony-bytes = { git="https://github.com/sankar-boro/loony", features = ["serde"] }
```

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `loony-bytes` by you, shall be licensed as MIT, without any additional
terms or conditions.
