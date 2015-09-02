use std::marker::PhantomData;

use ffi::*;

pub struct FilterGraph<'a> {
	ptr: *mut AVFilterGraph,

	_own: bool,
	_marker: PhantomData<&'a ()>,
}

impl<'a> FilterGraph<'a> {
	pub unsafe fn wrap(ptr: *mut AVFilterGraph) -> Self {
		FilterGraph { ptr: ptr, _own: false, _marker: PhantomData }
	}

	pub unsafe fn own(ptr: *mut AVFilterGraph) -> Self {
		FilterGraph { ptr: ptr, _own: true, _marker: PhantomData }
	}

	pub unsafe fn as_ptr(&self) -> *const AVFilterGraph {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFilterGraph {
		self.ptr
	}

	pub unsafe fn take(mut self) -> *mut AVFilterGraph {
		self._own = false;
		self.ptr
	}
}
