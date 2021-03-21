use std::{mem, ops::Deref};

use super::Stream;
use crate::{codec, ffi::*, format::context::common::Context, Dictionary, Rational};

pub struct StreamMut<'a> {
	context: &'a mut Context,
	index: usize,

	immutable: Stream<'a>,
}

impl<'a> StreamMut<'a> {
	pub unsafe fn wrap(context: &mut Context, index: usize) -> StreamMut<'_> {
		StreamMut {
			context: mem::transmute_copy(&context),
			index,

			immutable: Stream::wrap(mem::transmute_copy(&context), index),
		}
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVStream {
		*(*self.context.as_mut_ptr()).streams.add(self.index)
	}
}

impl<'a> StreamMut<'a> {
	pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
		unsafe {
			(*self.as_mut_ptr()).time_base = value.into().into();
		}
	}

	pub fn set_rate<R: Into<Rational>>(&mut self, value: R) {
		unsafe {
			av_stream_set_r_frame_rate(self.as_mut_ptr(), value.into().into());
		}
	}

	pub fn set_avg_frame_rate<R: Into<Rational>>(&mut self, value: R) {
		unsafe {
			(*self.as_mut_ptr()).avg_frame_rate = value.into().into();
		}
	}

	#[cfg(feature = "ffmpeg_3_1")]
	pub fn set_parameters<P: Into<codec::Parameters>>(&mut self, parameters: P) {
		let parameters = parameters.into();

		#[cfg(feature = "ffmpeg_3_1")]
		unsafe {
			avcodec_parameters_copy((*self.as_mut_ptr()).codecpar, parameters.as_ptr());
		}

		#[cfg(not(feature = "ffmpeg_3_1"))]
		unsafe {
			avcodec_parameters_copy((*self.as_mut_ptr()).codec, parameters.as_ptr());
		}
	}

	pub fn set_metadata(&mut self, metadata: Dictionary) {
		unsafe {
			let metadata = metadata.disown();
			(*self.as_mut_ptr()).metadata = metadata;
		}
	}
}

impl<'a> Deref for StreamMut<'a> {
	type Target = Stream<'a>;

	fn deref(&self) -> &Self::Target {
		&self.immutable
	}
}
