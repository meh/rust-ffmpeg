use std::marker::PhantomData;

use ffi::*;

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
