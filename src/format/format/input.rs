use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::*;

pub struct Input {
	ptr: *mut AVInputFormat,
}

impl Input {
	pub unsafe fn wrap(ptr: *mut AVInputFormat) -> Self {
		Input { ptr: ptr }
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
		unsafe {
			from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes())
		}
	}

	pub fn description(&self) -> &str {
		unsafe {
			from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes())
		}
	}

	pub fn extensions(&self) -> Vec<&str> {
		unsafe {
			let ptr = (*self.as_ptr()).extensions;

			if ptr.is_null() {
				Vec::new()
			}
			else {
				from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()).split(',').collect()
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
				from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()).split(',').collect()
			}
		}
	}

	pub fn seek_to_pts(&self) -> bool {
		unsafe {
			AVFMT_SEEK_TO_PTS == (*self.as_ptr()).flags & AVFMT_SEEK_TO_PTS
		}
	}
}
