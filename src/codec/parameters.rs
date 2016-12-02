use std::rc::Rc;

use ffi::*;

pub struct Parameters {
	ptr: *mut AVCodecParameters,
	owner: Option<Rc<Drop>>,
}

unsafe impl Send for Parameters {}

impl Parameters {
	pub unsafe fn wrap(ptr: *mut AVCodecParameters, owner: Option<Rc<Drop>>) -> Self {
		Parameters { ptr: ptr, owner: owner }
	}

	pub unsafe fn as_ptr(&self) -> *const AVCodecParameters {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVCodecParameters {
		self.ptr
	}
}

impl Parameters {
	pub fn new() -> Self {
		unsafe {
			Parameters { ptr: avcodec_parameters_alloc(), owner: None }
		}
	}

}

impl Drop for Parameters {
	fn drop(&mut self) {
		unsafe {
			if self.owner.is_none() {
				avcodec_parameters_free(&mut self.as_mut_ptr());
			}
		}
	}
}

impl Clone for Parameters {
	fn clone(&self) -> Self {
		let mut ctx = Parameters::new();
		ctx.clone_from(self);

		ctx
	}

	fn clone_from(&mut self, source: &Self) {
		unsafe {
			avcodec_parameters_copy(self.as_mut_ptr(), source.as_ptr());
		}
	}
}
