#![allow(non_camel_case_types)]
#![cfg_attr(feature = "unstable", feature(seek_stream_len))]
#![cfg_attr(feature = "cargo-clippy", allow(inline_always))]

#[macro_use]
extern crate bitflags;
pub extern crate ffmpeg_sys as sys;
#[cfg(feature = "image")]
extern crate image;
extern crate libc;

pub use crate::sys as ffi;

#[macro_use]
pub mod util;
pub use crate::util::{
	channel_layout::{self, ChannelLayout},
	chroma, color, dictionary,
	dictionary::{Mut as DictionaryMut, Owned as Dictionary, Ref as DictionaryRef},
	error::Error,
	frame::{self, Frame},
	mathematics::{self, rescale, Rescale, Rounding},
	media, option, picture,
	rational::{self, Rational},
	time,
};

#[cfg(feature = "format")]
pub mod format;
#[cfg(feature = "format")]
pub use crate::format::chapter::{Chapter, ChapterMut};
#[cfg(feature = "format")]
pub use crate::format::format::Format;
#[cfg(feature = "format")]
pub use crate::format::stream::{Stream, StreamMut};

#[cfg(feature = "codec")]
pub mod codec;
#[cfg(feature = "codec")]
pub use crate::codec::audio_service::AudioService;
#[cfg(feature = "codec")]
pub use crate::codec::codec::Codec;
#[cfg(feature = "codec")]
pub use crate::codec::discard::Discard;
#[cfg(feature = "codec")]
pub use crate::codec::field_order::FieldOrder;
#[cfg(feature = "codec")]
pub use crate::codec::packet::{self, Packet};
#[cfg(feature = "codec")]
pub use crate::codec::picture::Picture;
#[cfg(feature = "codec")]
pub use crate::codec::subtitle::{self, Subtitle};
#[cfg(feature = "codec")]
pub use crate::codec::threading;
#[cfg(feature = "codec")]
pub use crate::codec::{decoder, encoder};

#[cfg(feature = "device")]
pub mod device;

#[cfg(feature = "filter")]
pub mod filter;
#[cfg(feature = "filter")]
pub use crate::filter::Filter;

pub mod software;

fn init_util() {
	util::error::register_all();
	#[cfg(feature = "log")]
	util::log::register();
}

#[cfg(feature = "format")]
fn init_format() {
	format::register_all();
}

#[cfg(not(feature = "format"))]
fn init_format() {}

#[cfg(feature = "device")]
fn init_device() {
	device::register_all();
}

#[cfg(not(feature = "device"))]
fn init_device() {}

#[cfg(feature = "filter")]
fn init_filter() {
	filter::register_all();
}

#[cfg(not(feature = "filter"))]
fn init_filter() {}

pub fn init() -> Result<(), Error> {
	init_util();
	init_format();
	init_device();
	init_filter();

	Ok(())
}
