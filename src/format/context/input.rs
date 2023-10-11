use std::{
	ffi::CString,
	mem,
	ops::{Deref, DerefMut},
	ptr,
};

use super::{common::Context, destructor};
use crate::{
	ffi::*,
	format::{self, io::Io},
	util::range::Range,
	Codec, Error, Packet, Stream,
};

pub struct Input {
	ptr: *mut AVFormatContext,
	ctx: Context,
	// keeps IO proxy alive, no way to drop it otherwise
	_io: Option<Io>,
	has_found_streams: bool,
}

unsafe impl Send for Input {}

impl Input {
	pub unsafe fn wrap(ptr: *mut AVFormatContext) -> Self {
		Input {
			ptr,
			ctx: Context::wrap(ptr, destructor::Mode::Input),
			_io: None,
			has_found_streams: false,
		}
	}

	pub unsafe fn wrap_with(ptr: *mut AVFormatContext, io: Io) -> Self {
		Input {
			ptr,
			ctx: Context::wrap(ptr, destructor::Mode::Input),
			_io: Some(io),
			has_found_streams: false,
		}
	}

	pub unsafe fn as_ptr(&self) -> *const AVFormatContext {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFormatContext {
		self.ptr
	}
}

impl Input {
	pub fn format(&self) -> format::Input {
		unsafe { format::Input::wrap((*self.as_ptr()).iformat) }
	}

	pub fn video_codec(&self) -> Option<Codec> {
		unsafe {
			let ptr = (*self.as_ptr()).video_codec;

			if ptr.is_null() {
				None
			} else {
				Some(Codec::wrap(ptr))
			}
		}
	}

	pub fn audio_codec(&self) -> Option<Codec> {
		unsafe {
			let ptr = (*self.as_ptr()).audio_codec;

			if ptr.is_null() {
				None
			} else {
				Some(Codec::wrap(ptr))
			}
		}
	}

	pub fn subtitle_codec(&self) -> Option<Codec> {
		unsafe {
			let ptr = (*self.as_ptr()).subtitle_codec;

			if ptr.is_null() {
				None
			} else {
				Some(Codec::wrap(ptr))
			}
		}
	}

	pub fn data_codec(&self) -> Option<Codec> {
		unsafe {
			let ptr = (*self.as_ptr()).data_codec;

			if ptr.is_null() {
				None
			} else {
				Some(Codec::wrap(ptr))
			}
		}
	}

	pub fn find_stream_info(&mut self) -> Result<(), Error> {
		// avformat_find_stream_info crashes the app with a memory error if ran twice
		if self.has_found_streams {
			return Ok(());
		}

		unsafe {
			match avformat_find_stream_info(self.as_mut_ptr(), ptr::null_mut()) {
				r if r >= 0 => {
					self.has_found_streams = true;
					Ok(())
				}
				e => Err(Error::from(e)),
			}
		}
	}

	pub fn probe_score(&self) -> i32 {
		unsafe { (*self.as_ptr()).probe_score }
	}

	pub fn packets(&mut self) -> PacketIter<'_> {
		PacketIter::new(self)
	}

	pub fn pause(&mut self) -> Result<(), Error> {
		unsafe {
			match av_read_pause(self.as_mut_ptr()) {
				0 => Ok(()),
				e => Err(Error::from(e)),
			}
		}
	}

	pub fn play(&mut self) -> Result<(), Error> {
		unsafe {
			match av_read_play(self.as_mut_ptr()) {
				0 => Ok(()),
				e => Err(Error::from(e)),
			}
		}
	}

	pub fn seek<R: Range<i64>>(&mut self, ts: i64, range: R) -> Result<(), Error> {
		unsafe {
			match avformat_seek_file(
				self.as_mut_ptr(),
				-1,
				range.start().cloned().unwrap_or(i64::min_value()),
				ts,
				range.end().cloned().unwrap_or(i64::max_value()),
				0,
			) {
				s if s >= 0 => Ok(()),
				e => Err(Error::from(e)),
			}
		}
	}
}

impl Deref for Input {
	type Target = Context;

	fn deref(&self) -> &Self::Target {
		&self.ctx
	}
}

impl DerefMut for Input {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.ctx
	}
}

pub struct PacketIter<'a> {
	context: &'a mut Input,
}

impl<'a> PacketIter<'a> {
	pub fn new(context: &mut Input) -> PacketIter<'_> {
		PacketIter { context }
	}
}

impl<'a> Iterator for PacketIter<'a> {
	type Item = Result<(Stream<'a>, Packet), Error>;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		let mut packet = Packet::empty();

		match packet.read(self.context) {
			Err(Error::Eof) => None,
			Err(Error::Exit) => None,

			Ok(..) => unsafe {
				let context = mem::transmute::<&Context, &Context>(self.context);
				let stream = Stream::wrap(context, packet.stream());

				Some(Ok((stream, packet)))
			},

			Err(err) => Some(Err(err)),
		}
	}
}

pub fn dump(ctx: &Input, index: i32, url: Option<&str>) {
	let url = url.map(|u| CString::new(u).unwrap());

	unsafe {
		av_dump_format(
			ctx.as_ptr() as *mut _,
			index,
			url.unwrap_or_else(|| CString::new("").unwrap()).as_ptr(),
			0,
		);
	}
}
