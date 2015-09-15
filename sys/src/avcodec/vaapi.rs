use libc::{c_void, c_uint, uint8_t, uint32_t};

#[derive(Debug)]
#[repr(C)]
pub struct vaapi_context {
	pub display: *mut c_void,
	pub config_id: uint32_t,
	pub context_id: uint32_t,
	pub pic_param_buf_id: uint32_t, // XXX: #if FF_API_VAAPI_CONTEXT
	pub iq_matrix_buf_id: uint32_t, // XXX: #if FF_API_VAAPI_CONTEXT
	pub bitplane_buf_id: uint32_t, // XXX: #if FF_API_VAAPI_CONTEXT
	pub slice_buf_ids: *mut uint32_t, // XXX: #if FF_API_VAAPI_CONTEXT
	pub n_slice_buf_ids: c_uint, // XXX: #if FF_API_VAAPI_CONTEXT
	pub slice_buf_ids_alloc: c_uint, // XXX: #if FF_API_VAAPI_CONTEXT
	pub slice_params: *mut c_void, // XXX: #if FF_API_VAAPI_CONTEXT
	pub slice_param_size: c_uint, // XXX: #if FF_API_VAAPI_CONTEXT
	pub slice_params_alloc: c_uint, // XXX: #if FF_API_VAAPI_CONTEXT
	pub slice_count: c_uint, // XXX: #if FF_API_VAAPI_CONTEXT
	pub slice_data: *const uint8_t, // XXX: #if FF_API_VAAPI_CONTEXT
	pub slice_data_size: uint32_t, // XXX: #if FF_API_VAAPI_CONTEXT
}
