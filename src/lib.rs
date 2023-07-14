#![no_std]
#![doc = include_str!("../README.md")]

mod bytewise_table;

pub use crate::bytewise_table::crc32c;
