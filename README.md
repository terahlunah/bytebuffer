[![Crates.io](https://img.shields.io/crates/v/bytebuffer.svg)](https://crates.io/crates/bytebuffer)
[![docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/bytebuffer)
[![CI Checks](https://github.com/terahlunah/bytebuffer/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/terahlunah/bytebuffer/actions/workflows/rust.yml)

### A bytebuffer implementation for Rust

---

This crate provides an easy to use api to read / write data from / to a bunch of bytes.


```rust
use bytebuffer::ByteBuffer;

let mut buffer = ByteBuffer::new();
buffer.write_bytes(&vec![0x1, 0xFF, 0x45]);
buffer.write_u8(1);
buffer.write_i8(1);
buffer.write_u16(1);
buffer.write_i16(1);
buffer.write_u32(1);
buffer.write_i32(1);
buffer.write_u64(1);
buffer.write_i64(1);
buffer.write_f32(0.1);
buffer.write_f64(0.1);
buffer.write_string("Hello");
buffer.write_bit(true);
buffer.write_bits(4, 3);
buffer.flush_bits();

let data = buffer.into_bytes();

let mut buffer = ByteBuffer::from_vec(data);
let _ = buffer.read_bytes(3);
let _ = buffer.read_u8();
let _ = buffer.read_i8();
let _ = buffer.read_u16();
let _ = buffer.read_i16();
let _ = buffer.read_u32();
let _ = buffer.read_i32();
let _ = buffer.read_u64();
let _ = buffer.read_i64();
let _ = buffer.read_f32();
let _ = buffer.read_f64();
let _ = buffer.read_string();
let _ = buffer.read_bit();
let _ = buffer.read_bits(3);
```

---

### License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
