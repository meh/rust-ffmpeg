use std::{ptr, mem};
use std::ffi::CString;
use std::marker::PhantomData;

use ffi::*;
use ::{format, Frame, Error, ChannelLayout};

pub struct Context<'a> {
	ptr: *mut AVFilterContext,

	_own: bool,
	_marker: PhantomData<&'a ()>,
}

impl<'a> Context<'a> {
	pub unsafe fn wrap(ptr: *mut AVFilterContext) -> Self {
		Context { ptr: ptr, _own: false, _marker: PhantomData }
	}

	pub unsafe fn own(ptr: *mut AVFilterContext) -> Self {
		Context { ptr: ptr, _own: true, _marker: PhantomData }
	}

	pub unsafe fn as_ptr(&self) -> *const AVFilterContext {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFilterContext {
		self.ptr
	}

	pub unsafe fn take(mut self) -> *mut AVFilterContext {
		self._own = false;
		self.ptr
	}
}

impl<'a> Context<'a> {
	pub fn add_frame(&mut self, frame: &mut Frame) -> Result<(), Error> {
		unsafe {
			match av_buffersrc_add_frame_flags(self.as_mut_ptr(), frame.as_mut_ptr(), 0) {
				0 => Ok(()),
				e => Err(Error::from(e)),
			}
		}
	}

	pub fn frames(&mut self) -> FramesIter {
		unsafe {
			FramesIter::new(self.as_mut_ptr())
		}
	}

	fn set_opt<T>(&mut self, name: &str, value: T) {
		let name_cstr = CString::new(name).unwrap();
		let raw = &value as *const T;
		unsafe {
			av_opt_set_bin(self.as_mut_ptr() as *mut _, name_cstr.as_ptr(), raw as *const _,
				mem::size_of::<T>() as i32, AV_OPT_SEARCH_CHILDREN);
		}
	}

	pub fn set_pixel_format(&mut self, value: format::Pixel) {
        self.set_opt::<AVPixelFormat>("pix_fmts", value.into());
	}

	pub fn set_sample_format(&mut self, value: format::Sample) {
        self.set_opt::<AVSampleFormat>("sample_fmts", value.into());
	}

	pub fn set_sample_rate(&mut self, value: u32) {
        self.set_opt("sample_rates", value as i64);
	}

	pub fn set_channel_layout(&mut self, value: ChannelLayout) {
        self.set_opt("channel_layouts", value.bits() as i64);
	}
}

pub struct FramesIter<'a> {
	ptr: *mut AVFilterContext,

	_marker: PhantomData<&'a Context<'a>>,
}

impl<'a> FramesIter<'a> {
	pub fn new(ptr: *mut AVFilterContext) -> Self {
		FramesIter { ptr: ptr, _marker: PhantomData }
	}
}

impl<'a> Iterator for FramesIter<'a> {
	type Item = Frame;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		unsafe {
			let mut frame = Frame::empty();
			match av_buffersink_get_frame(self.ptr, frame.as_mut_ptr()) {
				s if s >= 0 => Some(frame),
				_ => None,
			}
		}
	}
}

impl<'a> Drop for Context<'a> {
	fn drop(&mut self) {
		unsafe {
			if self._own && self.as_ptr() != ptr::null() {
				avfilter_free(self.as_mut_ptr());
			}
		}
	}
}
