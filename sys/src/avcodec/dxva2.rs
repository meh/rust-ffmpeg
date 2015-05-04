use libc::{c_void, c_uint, uint64_t};

pub const FF_DXVA2_WORKAROUND_SCALING_LIST_ZIGZAG: uint64_t = 1;
pub const FF_DXVA2_WORKAROUND_INTEL_CLEARVIDEO:    uint64_t = 2;

#[derive(Debug)]
#[repr(C)]
pub struct dxva_context {
	pub decoder: *mut c_void,
	pub cfg: *const c_void,
	pub surface_count: c_uint,
	pub surface: *mut c_void,
	pub workaround: uint64_t,
	pub report_id: c_uint,
}
