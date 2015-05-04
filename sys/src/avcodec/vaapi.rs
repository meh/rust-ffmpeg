use libc::{c_void, c_uint, uint8_t, uint32_t};

#[derive(Debug)]
#[repr(C)]
pub struct vaapi_context {
	pub display: *mut c_void,
	pub config_id: uint32_t,
	pub context_id: uint32_t,
	pub pic_param_buf_id: uint32_t,
	pub iq_matrix_buf_id: uint32_t,
	pub bitplane_buf_id: uint32_t,
	pub slice_buf_ids: *mut uint32_t,
	pub n_slice_buf_ids: c_uint,
	pub slice_buf_ids_alloc: c_uint,
	pub slice_params: *mut c_void,
	pub slice_param_size: c_uint,
	pub slice_params_alloc: c_uint,
	pub slice_count: c_uint,
	pub slice_data: *const uint8_t,
	pub slice_data_size: uint32_t,
}
