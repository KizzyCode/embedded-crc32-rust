[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/embedded-crc32c-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/embedded-crc32c-rust)
[![docs.rs](https://docs.rs/embedded-crc32c/badge.svg)](https://docs.rs/embedded-crc32c)
[![crates.io](https://img.shields.io/crates/v/embedded-crc32c.svg)](https://crates.io/crates/embedded-crc32c)
[![Download numbers](https://img.shields.io/crates/d/embedded-crc32c.svg)](https://crates.io/crates/embedded-crc32c)
[![dependency status](https://deps.rs/crate/embedded-crc32c/latest/status.svg)](https://deps.rs/crate/embedded-crc32c)


# `embedded-crc32c`
A `const`, single-choice opinionated tiny CRC32 implementation.

## Why should I use this crate?
In most cases there's probably no good reason. However, why I wrote this crate: It's really tiny, portable and `no-std`
compatible and thus suitable embedded devices. Furthermore it's opinionated, offering only a single CRC implementation
(`CRC32C`; polynomial `0x1EDC6F41`). Also, the CRC32C implementation is `const`, which may be useful in some cases.
