use std::{
	ffi::{CStr, CString},
	io, ptr,
	str::from_utf8_unchecked,
};
use std::{fmt, rc::Rc};

use crate::{ffi::*, Error};

use crate::{codec::Parameters, packet};

pub struct Context {
	ptr: *mut AVBSFContext,
	owner: Option<Rc<dyn Drop>>,
}

unsafe impl Send for Context {}

impl Context {
	pub unsafe fn as_ptr(&self) -> *const AVBSFContext {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVBSFContext {
		self.ptr
	}
}

impl Context {
	pub fn new(filter: &str, parameters: &Parameters) -> Result<Self, Error> {
		unsafe {
			let filter = CString::new(filter).unwrap();
			let mut ptr: *mut AVBSFContext = ptr::null_mut();
			let ptr_ptr = &mut ptr as *mut *mut AVBSFContext;
			let filter = av_bsf_get_by_name(filter.as_ptr());

			if filter.is_null() {
				return Err(Error::BsfNotFound);
			}

			let result = av_bsf_alloc(filter, ptr_ptr);

			if result < 0 {
				return Err(Error::Io(io::Error::from_raw_os_error(-result)));
			}

			let result = avcodec_parameters_copy((*ptr).par_in, parameters.as_ptr());

			if result < 0 {
				return Err(Error::Io(io::Error::from_raw_os_error(-result)));
			}

			let result = av_bsf_init(ptr);

			if result < 0 {
				return Err(Error::Io(io::Error::from_raw_os_error(-result)));
			}

			Ok(Context { ptr, owner: None })
		}
	}

	pub fn parameters_out(&self) -> Parameters {
		let mut parameters = Parameters::new();
		unsafe {
			avcodec_parameters_copy(parameters.as_mut_ptr(), (*self.as_ptr()).par_out);
		}
		parameters
	}

	pub fn send_packet<P: packet::Mut>(&mut self, mut packet: P) -> Result<(), Error> {
		unsafe {
			match av_bsf_send_packet(self.as_mut_ptr(), packet.as_mut_ptr()) {
				e if e < 0 => Err(Error::from(e)),
				_ => Ok(()),
			}
		}
	}

	pub fn receive_packet<P: packet::Mut>(&mut self, packet: &mut P) -> Result<(), Error> {
		unsafe {
			match av_bsf_receive_packet(self.as_mut_ptr(), packet.as_mut_ptr()) {
				e if e < 0 => Err(Error::from(e)),
				_ => Ok(()),
			}
		}
	}

	pub fn flush(&mut self) {
		unsafe {
			av_bsf_flush(self.as_mut_ptr());
		}
	}

	pub fn name(&self) -> &str {
		unsafe { from_utf8_unchecked(CStr::from_ptr((*(*self.as_ptr()).filter).name).to_bytes()) }
	}
}

impl fmt::Debug for Context {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.name())
	}
}

impl Drop for Context {
	fn drop(&mut self) {
		unsafe {
			if self.owner.is_none() {
				av_bsf_free(&mut self.as_mut_ptr());
			}
		}
	}
}
