use std::ptr;
use std::marker::PhantomData;

use ffi::*;
use ::Error;

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

impl<'a> FilterGraph<'a> {
	pub fn new() -> Result<Self, Error> {
		let ptr = unsafe {
			avfilter_graph_alloc()
		};
		if ptr.is_null() {
			Err(Error::Unknown)
		}
		else {
			Ok(FilterGraph {
				ptr: ptr,
				_own: true,
				_marker: PhantomData
			})
		}
	}
}

impl<'a> Drop for FilterGraph<'a> {
	fn drop(&mut self) {
		unsafe {
			if self._own && self.as_ptr() != ptr::null() {
				avfilter_graph_free(&mut self.as_mut_ptr() as *mut _);
			}
		}
	}
}
