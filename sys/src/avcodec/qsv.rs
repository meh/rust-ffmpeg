use libc::{c_void, c_int};

#[derive(Debug)]
#[repr(C)]
pub struct AVQSVContext {
	pub session: *mut c_void,
	pub iopattern: c_int,
	pub ext_buffers: *mut *mut c_void,
	pub nb_ext_buffers: c_int,
}

extern {
	pub fn av_qsv_alloc_context() -> *mut AVQSVContext;
}
