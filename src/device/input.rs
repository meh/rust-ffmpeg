use std::ptr;

use crate::{ffi::*, format, Format};

pub struct AudioIter(*const AVInputFormat);

impl Iterator for AudioIter {
	type Item = Format;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		unsafe {
            #[cfg(feature = "ffmpeg_5_0")]
			let ptr = av_input_audio_device_next(self.0);

            #[cfg(not(feature = "ffmpeg_5_0"))]
			let ptr = av_input_audio_device_next(self.0.cast_mut());

			if ptr.is_null() && !self.0.is_null() {
				None
			}
			else {
				self.0 = ptr;

				Some(Format::Input(format::Input::wrap(ptr)))
			}
		}
	}
}

pub fn audio() -> AudioIter {
	AudioIter(ptr::null_mut())
}

pub struct VideoIter(*const AVInputFormat);

impl Iterator for VideoIter {
	type Item = Format;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		unsafe {
            #[cfg(feature = "ffmpeg_5_0")]
			let ptr = av_input_video_device_next(self.0);

            #[cfg(not(feature = "ffmpeg_5_0"))]
			let ptr = av_input_video_device_next(self.0.cast_mut());

			if ptr.is_null() && !self.0.is_null() {
				None
			}
			else {
				self.0 = ptr;

				Some(Format::Input(format::Input::wrap(ptr)))
			}
		}
	}
}

pub fn video() -> VideoIter {
	VideoIter(ptr::null_mut())
}
