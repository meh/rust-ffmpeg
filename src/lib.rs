#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]

#[macro_use]
extern crate bitflags;
pub extern crate ffmpeg_sys as sys;
#[cfg(feature = "image")]
extern crate image;
extern crate libc;

pub use crate::sys as ffi;

#[macro_use]
pub mod util;
pub use crate::util::channel_layout::{self, ChannelLayout};
pub use crate::util::chroma;
pub use crate::util::color;
pub use crate::util::dictionary;
pub use crate::util::dictionary::Mut as DictionaryMut;
pub use crate::util::dictionary::Owned as Dictionary;
pub use crate::util::dictionary::Ref as DictionaryRef;
pub use crate::util::error::{self, Error};
pub use crate::util::frame::{self, Frame};
pub use crate::util::log;
pub use crate::util::mathematics::{self, rescale, Rescale, Rounding};
pub use crate::util::media;
pub use crate::util::option;
pub use crate::util::picture;
pub use crate::util::rational::{self, Rational};
pub use crate::util::time;

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

fn init_error() {
    util::error::register_all();
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

#[cfg_attr(
    any(feature = "ffmpeg4", feature = "ffmpeg41", feature = "ffmpeg42"),
    deprecated(
        note = "features ffmpeg4/ffmpeg41/ffmpeg42/ffmpeg43 are now auto-detected \
        and will be removed in a future version"
    )
)]
pub fn init() -> Result<(), Error> {
    init_error();
    init_format();
    init_device();
    init_filter();

    Ok(())
}
