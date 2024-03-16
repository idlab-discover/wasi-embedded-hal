// mod bindings;
#![cfg_attr(not(feature = "use_std"), no_std)]

#[cfg(feature = "use_alloc")]
extern crate alloc;

// #[cfg(feature = "use_alloc")]
// use alloc::vec::Vec;

pub mod i2c;
