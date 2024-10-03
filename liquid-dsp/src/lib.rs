#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

mod complex;
mod error;

pub mod filter;
pub use error::ErrorKind;
