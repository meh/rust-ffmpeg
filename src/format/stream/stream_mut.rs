use std::{
	mem::{self, size_of},
	ops::Deref,
};

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
			context: mem::transmute::<&mut Context, &mut Context>(context),
			index,

			immutable: Stream::wrap(mem::transmute::<&Context, &Context>(context), index),
		}
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVStream {
		*(*self.context.as_mut_ptr()).streams.add(self.index)
	}
}

impl<'a> StreamMut<'a> {
	pub fn set_time_base<R: Into<Rational>>(&mut self, value: Option<R>) {
		unsafe {
			(*self.as_mut_ptr()).time_base = value.map(Into::into).unwrap_or(Rational::ZERO).into();
		}
	}

	pub fn set_rate<R: Into<Rational>>(&mut self, value: R) {
		unsafe {
			(*self.as_mut_ptr()).r_frame_rate = value.into().into();
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

	pub fn set_display_rotation(&mut self, angle: f64) {
		// Display matrix in libavcodec is [i32; 9] where each point is 16.16
		// fixed point, but all this is opaque here..
		const MATRIX_LEN: usize = 9 * size_of::<i32>();
		let mut matrix = [0u8; MATRIX_LEN];

		unsafe {
			av_display_rotation_set(matrix.as_mut_ptr() as *mut i32, angle);

			let mut data_size: usize = 0;
			let mut side_data = av_stream_get_side_data(
				self.as_mut_ptr(),
				AVPacketSideDataType::AV_PKT_DATA_DISPLAYMATRIX,
				&mut data_size,
			);
			if side_data.is_null() || data_size != MATRIX_LEN {
				side_data = av_stream_new_side_data(
					self.as_mut_ptr(),
					AVPacketSideDataType::AV_PKT_DATA_DISPLAYMATRIX,
					MATRIX_LEN,
				);
			}
			side_data.copy_from(matrix.as_ptr(), MATRIX_LEN);
		}
	}
}

impl<'a> Deref for StreamMut<'a> {
	type Target = Stream<'a>;

	fn deref(&self) -> &Self::Target {
		&self.immutable
	}
}
