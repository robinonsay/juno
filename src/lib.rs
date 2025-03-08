#![cfg_attr(not(feature = "std"), no_std)]

include!(concat!(env!("OUT_DIR"), "/constants.rs"));
extern crate alloc;
pub mod error;
pub mod msg;
pub mod math;
pub mod string;
pub mod hash;
pub mod collections;
