//! Implementation of [`embedded-hal`] traits for the bindings generated from the `wasi-i2c` WIT.`
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal
//! [`wasi-i2c`]: https://github.com/WebAssembly/wasi-i2c
//!
//! This crate provides support for an optional `no_std` environment.

#![cfg_attr(not(feature = "use_std"), no_std)]

#[cfg(feature = "use_alloc")]
extern crate alloc;

pub mod i2c;
