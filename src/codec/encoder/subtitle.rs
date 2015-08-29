use std::ops::{Deref, DerefMut};
use std::ptr;

use libc::c_int;
use ffi::*;

use super::Encoder as SuperEncoder;
use ::{Error, Dictionary, Codec};

pub struct Subtitle(pub SuperEncoder);

impl Subtitle {
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

impl Deref for Subtitle {
	type Target = SuperEncoder;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.0
	}
}

impl DerefMut for Subtitle {
	fn deref_mut(&mut self) -> &mut<Self as Deref>::Target {
		&mut self.0
	}
}

pub struct Encoder(pub Subtitle);

impl Encoder {
	pub fn encode(&mut self, subtitle: &::Subtitle, out: &mut [u8]) -> Result<bool, Error> {
		unsafe {
			match avcodec_encode_subtitle(self.0.as_mut_ptr(), out.as_mut_ptr(), out.len() as c_int, subtitle.as_ptr()) {
				e if e < 0 => Err(Error::from(e)),
				_          => Ok(true)
			}
		}
	}
}

impl Deref for Encoder {
	type Target = Subtitle;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.0
	}
}
