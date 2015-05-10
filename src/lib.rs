#![cfg_attr(all(test, feature = "unstable"), feature(test))]
#[cfg(test)] extern crate quickcheck;
#[cfg(all(test, feature = "unstable"))] extern crate test;

pub use log::{log2, log2_raw};

mod log;


#[doc(hidden)]
pub mod float;
