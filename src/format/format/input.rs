use std::{ffi::CStr, ptr, str::from_utf8_unchecked};

use crate::ffi::*;

#[derive(Copy, Clone)]
pub struct Input {
	ptr: *mut AVInputFormat,
}

unsafe impl Send for Input {}
unsafe impl Sync for Input {}

impl Input {
	pub unsafe fn wrap(ptr: *mut AVInputFormat) -> Self {
		Input { ptr }
	}

	pub unsafe fn as_ptr(&self) -> *const AVInputFormat {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVInputFormat {
		self.ptr
	}
}

impl Input {
	pub fn name(&self) -> &str {
		unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes()) }
	}

	pub fn description(&self) -> &str {
		unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).long_name).to_bytes()) }
	}

	pub fn extensions(&self) -> Vec<&str> {
		unsafe {
			let ptr = (*self.as_ptr()).extensions;

			if ptr.is_null() {
				Vec::new()
			}
			else {
				from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes())
					.split(',')
					.collect()
			}
		}
	}

	pub fn mime_types(&self) -> Vec<&str> {
		unsafe {
			let ptr = (*self.as_ptr()).mime_type;

			if ptr.is_null() {
				Vec::new()
			}
			else {
				from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes())
					.split(',')
					.collect()
			}
		}
	}
}

pub struct Iter {
	input: *mut AVInputFormat,
}

impl Iter {
	pub fn new() -> Self {
		Iter {
			input: ptr::null_mut(),
		}
	}
}

impl Default for Iter {
	fn default() -> Self {
		Self::new()
	}
}

impl Iterator for Iter {
	type Item = Input;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		unsafe {
			let ptr = av_iformat_next(self.input);

			if ptr.is_null() && !self.input.is_null() {
				None
			}
			else {
				self.input = ptr;
				Some(Input::wrap(ptr))
			}
		}
	}
}

pub fn all() -> Iter {
	Iter::new()
}

pub fn by_name(name: impl Into<String>) -> impl Iterator<Item = Input> {
	let name = name.into();
	all().filter(move |i| i.name() == name)
}

pub fn by_mime(mime: impl Into<String>) -> impl Iterator<Item = Input> {
	let mime = mime.into();
	all().filter(move |i| i.mime_types().contains(&mime.as_ref()))
}

pub fn by_extension(ext: impl Into<String>) -> impl Iterator<Item = Input> {
	let ext = ext.into();
	all().filter(move |i| i.extensions().contains(&ext.as_ref()))
}
