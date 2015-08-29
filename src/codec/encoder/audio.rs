use std::ops::{Deref, DerefMut};
use std::ptr;

use libc::c_int;
use ffi::*;

use super::Encoder as SuperEncoder;
use ::{Packet, Error, Dictionary, Codec};
use ::frame;

pub struct Audio(pub SuperEncoder);

impl Audio {
	pub fn open(mut self) -> Result<Encoder, Error> {
		unsafe {
			match avcodec_open2(self.as_mut_ptr(), ptr::null(), ptr::null_mut()) {
				0 => Ok(Encoder(self)),
				e => Err(Error::from(e))
			}
		}
	}

	pub fn open_as(mut self, codec: &Codec) -> Result<Encoder, Error> {
		if codec.is_encoder() {
			unsafe {
				match avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), ptr::null_mut()) {
					0 => Ok(Encoder(self)),
					e => Err(Error::from(e))
				}
			}
		}
		else {
			Err(Error::InvalidData)
		}
	}

	pub fn open_as_with(mut self, codec: &Codec, options: Dictionary) -> Result<Encoder, Error> {
		if codec.is_encoder() {
			unsafe {
				match avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), &mut options.take()) {
					0 => Ok(Encoder(self)),
					e => Err(Error::from(e))
				}
			}
		}
		else {
			Err(Error::InvalidData)
		}
	}
}

impl Deref for Audio {
	type Target = SuperEncoder;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.0
	}
}

impl DerefMut for Audio {
	fn deref_mut(&mut self) -> &mut<Self as Deref>::Target {
		&mut self.0
	}
}

pub struct Encoder(pub Audio);

impl Encoder {
	pub fn encode(&mut self, frame: &frame::Audio, out: &mut Packet) -> Result<bool, Error> {
		unsafe {
			let mut got: c_int = 0;

			match avcodec_encode_audio2(self.0.as_mut_ptr(), out.as_mut_ptr(), frame.as_ptr(), &mut got) {
				e if e < 0 => Err(Error::from(e)),
				_		   => Ok(got != 0)
			}
		}
	}
}

impl Deref for Encoder {
	type Target = Audio;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.0
	}
}
