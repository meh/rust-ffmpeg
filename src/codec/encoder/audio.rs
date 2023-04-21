use std::{
	ops::{Deref, DerefMut},
	ptr,
};

use libc::c_int;

use super::Encoder as Super;
use crate::{
	codec::{traits, Context},
	ffi::*,
	util::format,
	ChannelLayout, Dictionary, Error,
};

pub struct Audio(pub Super);

impl Audio {
	pub fn open(mut self) -> Result<Encoder, Error> {
		unsafe {
			match avcodec_open2(self.as_mut_ptr(), ptr::null(), ptr::null_mut()) {
				0 => Ok(Encoder(self)),
				e => Err(Error::from(e)),
			}
		}
	}

	pub fn open_as<E: traits::Encoder>(mut self, codec: E) -> Result<Encoder, Error> {
		unsafe {
			if let Some(codec) = codec.encoder() {
				match avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), ptr::null_mut()) {
					0 => Ok(Encoder(self)),
					e => Err(Error::from(e)),
				}
			}
			else {
				Err(Error::EncoderNotFound)
			}
		}
	}

	pub fn open_with(mut self, options: Dictionary) -> Result<Encoder, Error> {
		unsafe {
			let mut opts = options.disown();
			let res = avcodec_open2(self.as_mut_ptr(), ptr::null(), &mut opts);

			Dictionary::own(opts);

			match res {
				0 => Ok(Encoder(self)),
				e => Err(Error::from(e)),
			}
		}
	}

	pub fn open_as_with<E: traits::Encoder>(mut self, codec: E, options: Dictionary) -> Result<Encoder, Error> {
		unsafe {
			if let Some(codec) = codec.encoder() {
				let mut opts = options.disown();
				let res = avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), &mut opts);

				Dictionary::own(opts);

				match res {
					0 => Ok(Encoder(self)),
					e => Err(Error::from(e)),
				}
			}
			else {
				Err(Error::EncoderNotFound)
			}
		}
	}

	pub fn set_sample_rate(&mut self, rate: u32) {
		unsafe {
			(*self.as_mut_ptr()).sample_rate = rate as c_int;
		}
	}

	pub fn sample_rate(&self) -> u32 {
		unsafe { (*self.as_ptr()).sample_rate as u32 }
	}

	pub fn set_format(&mut self, value: format::Sample) {
		unsafe {
			(*self.as_mut_ptr()).sample_fmt = value.into();
		}
	}

	pub fn format(&self) -> format::Sample {
		unsafe { format::Sample::from((*self.as_ptr()).sample_fmt) }
	}

	pub fn set_channel_layout(&mut self, value: ChannelLayout) {
		unsafe {
			(*self.as_mut_ptr()).ch_layout = value.into();
		}
	}

	pub fn channel_layout(&self) -> ChannelLayout {
		unsafe { (*self.as_ptr()).ch_layout.into() }
	}

	pub fn set_channels(&mut self, value: i32) {
		unsafe {
			(*self.as_mut_ptr()).channels = value;
		}
	}

	pub fn channels(&self) -> u16 {
		unsafe { (*self.as_ptr()).channels as u16 }
	}
}

impl Deref for Audio {
	type Target = Super;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.0
	}
}

impl DerefMut for Audio {
	fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
		&mut self.0
	}
}

impl AsRef<Context> for Audio {
	fn as_ref(&self) -> &Context {
		self
	}
}

impl AsMut<Context> for Audio {
	fn as_mut(&mut self) -> &mut Context {
		&mut self.0
	}
}

pub struct Encoder(pub Audio);

impl Encoder {
	pub fn frame_size(&self) -> u32 {
		unsafe { (*self.as_ptr()).frame_size as u32 }
	}
}

impl Deref for Encoder {
	type Target = Audio;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.0
	}
}

impl DerefMut for Encoder {
	fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
		&mut self.0
	}
}

impl AsRef<Context> for Encoder {
	fn as_ref(&self) -> &Context {
		self
	}
}

impl AsMut<Context> for Encoder {
	fn as_mut(&mut self) -> &mut Context {
		&mut self.0
	}
}
