use std::marker::PhantomData;
use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::*;
use super::{Id, Context};
use ::media;
use ::{Error, Rational, ChannelLayout};
use ::format::{Pixel, Sample};
use ::codec::context::Opened;

pub struct Codec<'a> {
	ptr: *mut AVCodec,

	_marker: PhantomData<&'a ()>,
}

impl<'a> Codec<'a> {
	pub unsafe fn wrap(ptr: *mut AVCodec) -> Self {
		Codec { ptr: ptr, _marker: PhantomData }
	}

	pub unsafe fn as_ptr(&self) -> *const AVCodec {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVCodec {
		self.ptr
	}
}

impl<'a> Codec<'a> {
	pub fn open(&self) -> Result<Opened, Error> {
		Context::new().open(self)
	}

	pub fn is_encoder(&self) -> bool {
		unsafe {
			av_codec_is_encoder(self.as_ptr()) != 0
		}
	}

	pub fn is_decoder(&self) -> bool {
		unsafe {
			av_codec_is_decoder(self.as_ptr()) != 0
		}
	}

	pub fn name(&self) -> &str {
		unsafe {
			from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes())
		}
	}

	pub fn description(&self) -> &str {
		unsafe {
			from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).long_name).to_bytes())
		}
	}

	pub fn medium(&self) -> media::Type {
		unsafe {
			media::Type::from((*self.as_ptr()).kind)
		}
	}

	pub fn id(&self) -> Id {
		unsafe {
			Id::from((*self.as_ptr()).id)
		}
	}

	// capabilities

	pub fn framerates(&self) -> FramerateIter {
		unsafe {
			FramerateIter::new((*self.as_ptr()).supported_framerates)
		}
	}

	pub fn pixel_formats(&self) -> PixelFormatIter {
		unsafe {
			PixelFormatIter::new((*self.as_ptr()).pix_fmts)
		}
	}

	pub fn samplerates(&self) -> SamplerateIter {
		unsafe {
			SamplerateIter::new((*self.as_ptr()).supported_samplerates)
		}
	}

	pub fn sample_formats(&self) -> SampleFormatIter {
		unsafe {
			SampleFormatIter::new((*self.as_ptr()).sample_fmts)
		}
	}

	pub fn channel_layouts(&self) -> ChannelLayoutIter {
		unsafe {
			ChannelLayoutIter::new((*self.as_ptr()).channel_layouts)
		}
	}

	pub fn max_lowres(&self) -> i32 {
		unsafe {
			av_codec_get_max_lowres(self.as_ptr())
		}
	}

	// profiles
}

pub struct FramerateIter <'a> {
	ptr: *const AVRational,

	_marker: PhantomData<&'a Codec<'a>>,
}

impl<'a> FramerateIter<'a> {
	pub fn new(ptr: *const AVRational) -> Self {
		FramerateIter { ptr: ptr, _marker: PhantomData }
	}
}

impl<'a> Iterator for FramerateIter<'a> {
	type Item = Rational;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		unsafe {
			if !self.ptr.is_null() && (*self.ptr) != (AVRational { num: 0, den: 0 }) {
				let element = self.ptr;
				self.ptr = self.ptr.offset(1);
				Some(Rational::from(*element))
			} else {
				None
			}
		}
	}
}

pub struct PixelFormatIter <'a> {
	ptr: *const AVPixelFormat,

	_marker: PhantomData<&'a Codec<'a>>,
}

impl<'a> PixelFormatIter<'a> {
	pub fn new(ptr: *const AVPixelFormat) -> Self {
		PixelFormatIter { ptr: ptr, _marker: PhantomData }
	}
}

impl<'a> Iterator for PixelFormatIter<'a> {
	type Item = Pixel;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		unsafe {
			if !self.ptr.is_null() && (*self.ptr) != AVPixelFormat::AV_PIX_FMT_NONE {
				let element = self.ptr;
				self.ptr = self.ptr.offset(1);
				Some(Pixel::from(*element))
			} else {
				None
			}
		}
	}
}

pub struct SamplerateIter <'a> {
	ptr: *const i32,

	_marker: PhantomData<&'a Codec<'a>>,
}

impl<'a> SamplerateIter<'a> {
	pub fn new(ptr: *const i32) -> Self {
		SamplerateIter { ptr: ptr, _marker: PhantomData }
	}
}

impl<'a> Iterator for SamplerateIter<'a> {
	type Item = i32;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		unsafe {
			if !self.ptr.is_null() && (*self.ptr) != 0 {
				let element = self.ptr;
				self.ptr = self.ptr.offset(1);
				Some((*element))
			} else {
				None
			}
		}
	}
}

pub struct SampleFormatIter <'a> {
	ptr: *const AVSampleFormat,

	_marker: PhantomData<&'a Codec<'a>>,
}

impl<'a> SampleFormatIter<'a> {
	pub fn new(ptr: *const AVSampleFormat) -> Self {
		SampleFormatIter { ptr: ptr, _marker: PhantomData }
	}
}

impl<'a> Iterator for SampleFormatIter<'a> {
	type Item = Sample;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		unsafe {
			if !self.ptr.is_null() && (*self.ptr) != AVSampleFormat::AV_SAMPLE_FMT_NONE {
				let element = self.ptr;
				self.ptr = self.ptr.offset(1);
				Some(Sample::from(*element))
			} else {
				None
			}
		}
	}
}

pub struct ChannelLayoutIter <'a> {
	ptr: *const u64,

	_marker: PhantomData<&'a Codec<'a>>,
}

impl<'a> ChannelLayoutIter<'a> {
	pub fn new(ptr: *const u64) -> Self {
		ChannelLayoutIter { ptr: ptr, _marker: PhantomData }
	}
}

impl<'a> Iterator for ChannelLayoutIter<'a> {
	type Item = ChannelLayout;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		unsafe {
			if !self.ptr.is_null() && (*self.ptr) != 0 {
				let element = self.ptr;
				self.ptr = self.ptr.offset(1);
				Some(ChannelLayout::from_bits_truncate(*element))
			} else {
				None
			}
		}
	}
}
