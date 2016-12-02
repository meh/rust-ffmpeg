use std::ops::{Deref, DerefMut};

use libc::c_int;
use ffi::*;

use super::Opened;
use ::{Error, AudioService, ChannelLayout};
use ::packet::{self, Mut};
use ::frame;
use ::util::format;
use ::codec::Context;

pub struct Audio(pub Opened);

impl Audio {
	pub fn decode<P: packet::Ref>(&mut self, packet: &P, out: &mut frame::Audio) -> Result<bool, Error> {
		unsafe {
			let mut got: c_int = 0;

			match avcodec_decode_audio4(self.as_mut_ptr(), out.as_mut_ptr(), &mut got, packet.as_ptr()) {
				e if e < 0 => Err(Error::from(e)),
				_          => Ok(got != 0)
			}
		}
	}

	pub fn decode_iter<'a, 'b>(&'a mut self, packet: &'b mut packet::Packet) -> AudioFrameIter<'a, 'b> {
		AudioFrameIter {
			audio: self,
			packet: packet,
		}
	}

	pub fn rate(&self) -> u32 {
		unsafe {
			(*self.as_ptr()).sample_rate as u32
		}
	}

	pub fn channels(&self) -> u16 {
		unsafe {
			(*self.as_ptr()).channels as u16
		}
	}

	pub fn format(&self) -> format::Sample {
		unsafe {
			format::Sample::from((*self.as_ptr()).sample_fmt)
		}
	}

	pub fn request_format(&mut self, value: format::Sample) {
		unsafe {
			(*self.as_mut_ptr()).request_sample_fmt = value.into();
		}
	}

	pub fn frames(&self) -> usize {
		unsafe {
			(*self.as_ptr()).frame_number as usize
		}
	}

	pub fn align(&self) -> usize {
		unsafe {
			(*self.as_ptr()).block_align as usize
		}
	}

	pub fn channel_layout(&self) -> ChannelLayout {
		unsafe {
			ChannelLayout::from_bits_truncate((*self.as_ptr()).channel_layout)
		}
	}

	pub fn set_channel_layout(&mut self, value: ChannelLayout) {
		unsafe {
			(*self.as_mut_ptr()).channel_layout = value.bits();
		}
	}

	pub fn request_channel_layout(&mut self, value: ChannelLayout) {
		unsafe {
			(*self.as_mut_ptr()).request_channel_layout = value.bits();
		}
	}

	pub fn audio_service(&mut self) -> AudioService {
		unsafe {
			AudioService::from((*self.as_mut_ptr()).audio_service_type)
		}
	}

	pub fn max_bit_rate(&self) -> usize {
		unsafe {
			(*self.as_ptr()).rc_max_rate as usize
		}
	}

	pub fn frame_size(&self) -> u32 {
		unsafe {
			(*self.as_ptr()).frame_size as u32
		}
	}

	pub fn frame_start(&self) -> Option<usize> {
		unsafe {
			match (*self.as_ptr()).timecode_frame_start {
				-1 => None,
				n  => Some(n as usize)
			}
		}
	}
}

impl Deref for Audio {
	type Target = Opened;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.0
	}
}

impl DerefMut for Audio {
	fn deref_mut(&mut self) -> &mut<Self as Deref>::Target {
		&mut self.0
	}
}

impl AsRef<Context> for Audio {
	fn as_ref(&self) -> &Context {
		&self
	}
}

impl AsMut<Context> for Audio {
	fn as_mut(&mut self) -> &mut Context {
		&mut self.0
	}
}

pub struct AudioFrameIter<'a, 'b> {
	audio: &'a mut Audio,
	packet: &'b mut packet::Packet,
}

impl<'a, 'b> Iterator for AudioFrameIter<'a, 'b> {
    type Item = Result<Option<frame::Audio>, Error>;

    fn next(&mut self) -> Option<Result<Option<frame::Audio>, Error>> {
		unsafe {
			if !self.packet.is_empty() {
				let mut out = frame::Audio::empty();
				let mut got: c_int = 0;
				let packet = self.packet.as_mut_ptr();

				match avcodec_decode_audio4(self.audio.as_mut_ptr(), out.as_mut_ptr(), &mut got, packet) {
					e if e < 0 => Some(Err(Error::from(e))),
					n => {
						(*packet).data = (*packet).data.offset(n as isize);
						(*packet).size -= n;

						if got != 0 {
							Some(Ok(Some(out)))
						} else {
							Some(Ok(None))
						}
					}
				}
			} else {
				None
			}
		}
	}
}
