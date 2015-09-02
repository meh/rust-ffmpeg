use std::ffi::CString;
use std::marker::PhantomData;

use ffi::*;
use ::Error;

pub struct Filter<'a> {
	ptr: *mut AVFilter,

	_own: bool,
	_marker: PhantomData<&'a ()>,
}

impl<'a> Filter<'a> {
	pub unsafe fn wrap(ptr: *mut AVFilter) -> Self {
		Filter { ptr: ptr, _own: false, _marker: PhantomData }
	}

	pub unsafe fn own(ptr: *mut AVFilter) -> Self {
		Filter { ptr: ptr, _own: true, _marker: PhantomData }
	}

	pub unsafe fn as_ptr(&self) -> *const AVFilter {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFilter {
		self.ptr
	}

	pub unsafe fn take(mut self) -> *mut AVFilter {
		self._own = false;
		self.ptr
	}
}

impl<'a> Filter<'a> {
	pub fn get_by_name(name: &str) -> Result<Self, Error> {
		let name_cstr = CString::new(name).unwrap();
		let ptr = unsafe {
			avfilter_get_by_name(name_cstr.as_ptr())
		};
		if !ptr.is_null() {
			// there seems to be no way to deallocate a `AVFilter`
			Ok(Filter {
				ptr: ptr,
				_own: false,
				_marker: PhantomData,
			})
		}
		else {
			Err(Error::InvalidData)
		}
	}
}
