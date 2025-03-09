#![cfg_attr(not(feature = "std"), no_std)]

include!(concat!(env!("OUT_DIR"), "/constants.rs"));
extern crate alloc;
pub mod error;
pub mod string;
pub mod hash;
pub mod collections;
pub mod dds;
