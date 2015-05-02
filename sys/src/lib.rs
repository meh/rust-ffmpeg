#![feature(libc)]
#![allow(non_camel_case_types, raw_pointer_derive, non_snake_case)]

extern crate libc;

#[macro_use]
mod avutil;
pub use avutil::*;
