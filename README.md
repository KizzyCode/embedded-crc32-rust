[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)


# `embedded-crc32c`
A `const`, single-choice opinionated tiny CRC32 implementation.

## Why should I use this crate?
In most cases there's probably no good reason. However, why I wrote this crate: It's really tiny, portable and `no-std`
compatible and thus suitable embedded devices. Furthermore it's opinionated, offering only a single CRC implementation
(`CRC32C`; polynomial `0x1EDC6F41`). Also, the CRC32C implementation is `const`, which may be useful in some cases.
