#![feature(libc)]
#![allow(non_camel_case_types, raw_pointer_derive, non_snake_case, non_upper_case_globals)]

extern crate libc;

#[macro_use]
mod avutil;
pub use avutil::*;

mod avcodec;
pub use avcodec::*;

mod avdevice;
pub use avdevice::*;

mod avformat;
pub use avformat::*;

mod swscale;
pub use swscale::*;
