use std::ops::{Deref, DerefMut};

use libc::{c_int, int64_t};
use codec::Context;
use ::{Error, Rational, media};
use super::{video, audio, subtitle};

pub struct Encoder(pub Context);

impl Encoder {
	pub fn video(mut self) -> Result<video::Video, Error> {
		match self.medium() {
			media::Type::Unknown => {
				unsafe {
					(*self.as_mut_ptr()).codec_type = media::Type::Video.into();
				}

				Ok(video::Video(self))
			}

			media::Type::Video => {
				Ok(video::Video(self))
			}

			_ => {
				Err(Error::InvalidData)
			}
		}
	}

	pub fn audio(mut self) -> Result<audio::Audio, Error> {
		match self.medium() {
			media::Type::Unknown => {
				unsafe {
					(*self.as_mut_ptr()).codec_type = media::Type::Audio.into();
				}

				Ok(audio::Audio(self))
			}

			media::Type::Audio => {
				Ok(audio::Audio(self))
			}

			_ => {
				Err(Error::InvalidData)
			}
		}
	}

	pub fn subtitle(mut self) -> Result<subtitle::Subtitle, Error> {
		match self.medium() {
			media::Type::Unknown => {
				unsafe {
					(*self.as_mut_ptr()).codec_type = media::Type::Subtitle.into();
				}

				Ok(subtitle::Subtitle(self))
			}

			media::Type::Subtitle => {
				Ok(subtitle::Subtitle(self))
			}

			_ => {
				Err(Error::InvalidData)
			}
		}
	}

	pub fn set_bit_rate(&mut self, value: usize) {
		unsafe {
			(*self.as_mut_ptr()).bit_rate = value as int64_t;
		}
	}

	pub fn set_max_bit_rate(&mut self, value: usize) {
		unsafe {
			(*self.as_mut_ptr()).rc_max_rate = value as c_int;
		}
	}

	pub fn set_tolerance(&mut self, value: usize) {
		unsafe {
			(*self.as_mut_ptr()).bit_rate_tolerance = value as c_int;
		}
	}

	pub fn set_quality(&mut self, value: usize) {
		unsafe {
			(*self.as_mut_ptr()).global_quality = value as c_int;
		}
	}

	pub fn set_compression(&mut self, value: Option<usize>) {
		unsafe {
			if let Some(value) = value {
				(*self.as_mut_ptr()).compression_level = value as c_int;
			}
			else {
				(*self.as_mut_ptr()).compression_level = -1;
			}
		}
	}

	pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
		unsafe {
			(*self.as_mut_ptr()).time_base = value.into().into();
		}
	}

	pub fn set_frame_rate<R: Into<Rational>>(&mut self, value: Option<R>) {
		unsafe {
			if let Some(value) = value {
				(*self.as_mut_ptr()).framerate = value.into().into();
			}
			else {
				(*self.as_mut_ptr()).framerate.num = 0;
				(*self.as_mut_ptr()).framerate.den = 1;
			}
		}
	}
}

impl Deref for Encoder {
	type Target = Context;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.0
	}
}

impl DerefMut for Encoder {
	fn deref_mut(&mut self) -> &mut<Self as Deref>::Target {
		&mut self.0
	}
}
