pub mod video;
pub use self::video::Video;

pub mod audio;
pub use self::audio::Audio;

pub mod subtitle;
pub use self::subtitle::Subtitle;

pub mod slice;

pub mod conceal;
pub use self::conceal::Conceal;

pub mod check;
pub use self::check::Check;

pub mod opened;
pub use self::opened::Opened;

use std::ptr;
use std::ffi::CString;
use std::ops::{Deref, DerefMut};

use ffi::*;
use super::{Id, Context};
use ::{Codec, Error, Dictionary};

pub struct Decoder(pub Context);

impl Decoder {
	pub fn open(mut self, codec: &Codec) -> Result<Opened, Error> {
        if codec.is_decoder() {
            unsafe {
                match avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), ptr::null_mut()) {
                    0 => Ok(Opened(self)),
                    e => Err(Error::from(e))
                }
            }
        }
        else {
            Err(Error::InvalidData)
        }
	}

	pub fn open_with(mut self, codec: &Codec, options: Dictionary) -> Result<Opened, Error> {
        if codec.is_decoder() {
            unsafe {
                match avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), &mut options.take()) {
                    0 => Ok(Opened(self)),
                    e => Err(Error::from(e))
                }
            }
        }
        else {
            Err(Error::InvalidData)
        }
	}

	pub fn video(self) -> Result<Video, Error> {
        if let Some(ref codec) = find(self.id()) {
            self.open(codec).and_then(|o| o.video())
        }
        else {
            Err(Error::DecoderNotFound)
        }
	}

	pub fn audio(self) -> Result<Audio, Error> {
        if let Some(ref codec) = find(self.id()) {
            self.open(codec).and_then(|o| o.audio())
        }
        else {
            Err(Error::DecoderNotFound)
        }
	}

	pub fn subtitle(self) -> Result<Subtitle, Error> {
        if let Some(ref codec) = find(self.id()) {
            self.open(codec).and_then(|o| o.subtitle())
        }
        else {
            Err(Error::DecoderNotFound)
        }
	}
}

impl Deref for Decoder {
	type Target = Context;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.0
	}
}

impl DerefMut for Decoder {
	fn deref_mut(&mut self) -> &mut<Self as Deref>::Target {
		&mut self.0
	}
}

pub fn find(id: Id) -> Option<Codec<'static>> {
	unsafe {
		let ptr = avcodec_find_decoder(id.into());

		if ptr.is_null() {
			None
		}
		else {
			Some(Codec::wrap(ptr))
		}
	}
}

pub fn find_by_name(name: &str) -> Option<Codec<'static>> {
	unsafe {
		let ptr = avcodec_find_decoder_by_name(CString::new(name).unwrap().as_ptr());

		if ptr.is_null() {
			None
		}
		else {
			Some(Codec::wrap(ptr))
		}
	}
}
