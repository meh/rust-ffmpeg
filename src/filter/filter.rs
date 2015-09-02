use std::marker::PhantomData;

use ffi::*;

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
