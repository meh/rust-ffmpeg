use std::ptr;
use std::ffi::CString;
use std::marker::PhantomData;

use ffi::*;
use ::Error;
use super::Context;

pub struct InOut<'a> {
	ptr: *mut AVFilterInOut,

	_own: bool,
	_marker: PhantomData<&'a ()>,
}

impl<'a> InOut<'a> {
	pub unsafe fn wrap(ptr: *mut AVFilterInOut) -> Self {
		InOut { ptr: ptr, _own: false, _marker: PhantomData }
	}

	pub unsafe fn own(ptr: *mut AVFilterInOut) -> Self {
		InOut { ptr: ptr, _own: true, _marker: PhantomData }
	}

	pub unsafe fn as_ptr(&self) -> *const AVFilterInOut {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFilterInOut {
		self.ptr
	}

	pub unsafe fn take(mut self) -> *mut AVFilterInOut {
		self._own = false;
		self.ptr
	}
}

impl<'a> InOut<'a> {
	pub fn new(name: &str, context: &mut Context, pad_idx: i32) -> Result<Self, Error> {
		let ptr = unsafe { avfilter_inout_alloc() };
		let name_cstr = CString::new(name).unwrap();
		if !ptr.is_null() {
			unsafe {
				(*ptr).name = av_strdup(name_cstr.as_ptr());
				(*ptr).filter_ctx = context.as_mut_ptr();
				(*ptr).pad_idx = pad_idx;
				(*ptr).next = ptr::null_mut();
			}
			Ok(InOut {
				ptr: ptr,
				_own: true,
				_marker: PhantomData,
			})
		}
		else {
			Err(Error::Unknown)
		}
	}
}

impl<'a> Drop for InOut<'a> {
	fn drop(&mut self) {
		unsafe {
			if self._own && self.as_ptr() != ptr::null() {
				avfilter_inout_free(&mut self.as_mut_ptr() as *mut _);
			}
		}
	}
}
