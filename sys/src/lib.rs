#![allow(non_camel_case_types, raw_pointer_derive, non_snake_case, non_upper_case_globals)]

extern crate libc;

#[macro_use]
mod avutil;
pub use avutil::*;

#[cfg(feature = "avcodec")]
mod avcodec;
#[cfg(feature = "avcodec")]
pub use avcodec::*;

#[cfg(feature = "avdevice")]
mod avdevice;
#[cfg(feature = "avdevice")]
pub use avdevice::*;

#[cfg(feature = "avformat")]
mod avformat;
#[cfg(feature = "avformat")]
pub use avformat::*;

#[cfg(feature = "avfilter")]
mod avfilter;
#[cfg(feature = "avfilter")]
pub use avfilter::*;

#[cfg(feature = "avresample")]
mod avresample;
#[cfg(feature = "avresample")]
pub use avresample::*;

#[cfg(feature = "postproc")]
mod postproc;
#[cfg(feature = "postproc")]
pub use postproc::*;

#[cfg(feature = "swresample")]
mod swresample;
#[cfg(feature = "swresample")]
pub use swresample::*;

#[cfg(feature = "swscale")]
mod swscale;
#[cfg(feature = "swscale")]
pub use swscale::*;
