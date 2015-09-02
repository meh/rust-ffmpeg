use std::ptr;
use std::ffi::CString;
use std::marker::PhantomData;

use ffi::*;
use ::Error;
use super::{Context, Filter, InOut};

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

	pub fn create_filter<'b>(&mut self, filter: &Filter, name: &str, args: &str) -> Result<Context<'b>, Error> {
		let mut context = ptr::null_mut::<AVFilterContext>();
		let name_cstr = CString::new(name).unwrap();
		let args_cstr = CString::new(args).unwrap();
		let ret = unsafe {
			avfilter_graph_create_filter(&mut context as *mut _, filter.as_ptr(),
				name_cstr.as_ptr(), args_cstr.as_ptr(), ptr::null_mut(), self.as_mut_ptr())
		};
		if ret >= 0 {
			unsafe { Ok(Context::wrap(context)) }
		}
		else {
			Err(Error::from(ret))
		}
	}

	pub fn create_filter_by_name<'b>(&mut self, filter_name: &str, name: &str, args: &str) -> Result<Context<'b>, Error> {
		let filter = try!(Filter::get_by_name(filter_name));
		self.create_filter(&filter, name, args)
	}

	pub fn parse_ptr(&mut self, filters: &str, inputs: InOut, outputs: InOut) -> Result<(), Error> {
		let filters_cstr = CString::new(filters).unwrap();
		unsafe {
		    let mut inpts = inputs.take();
		    let mut otpts = outputs.take();
		    let ret = avfilter_graph_parse_ptr(self.as_mut_ptr(), filters_cstr.as_ptr(),
				&mut inpts, &mut otpts, ptr::null_mut());

		    InOut::own(inpts);
		    InOut::own(otpts);

		    if ret >= 0 {
			    Ok(())
		    }
			else {
			    Err(Error::from(ret))
		    }
		}
	}

	pub fn config(&mut self) -> Result<(), Error> {
		let ret = unsafe {
			avfilter_graph_config(self.as_mut_ptr(), ptr::null_mut())
		};
		if ret >= 0 {
			Ok(())
		}
		else {
			Err(Error::from(ret))
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
