use libc::{c_void, c_int, uint8_t, uint32_t};
use super::codec::AVCodecContext;

#[derive(Debug)]
#[repr(C)]
pub struct vda_context {
	pub decoder: *mut c_void,
	pub cv_buffer: *mut c_void,
	pub use_sync_decoding: c_int,
	pub width: c_int,
	pub height: c_int,
	pub format: c_int,
	pub cv_pix_fmt_type: i32,
	pub priv_bitstream: *mut uint8_t,
	pub priv_bitstream_size: c_int,
	pub priv_allocated_size: c_int,
	pub use_ref_buffer: c_int,
}

#[repr(C)]
pub struct AVVDAContext {
	pub decoder: *mut c_void,
	pub output_callback: extern fn(*mut c_void, *mut c_void, i32, uint32_t, *mut c_void),
}

extern {
	pub fn ff_vda_create_decoder(vda_ctx: *mut vda_context, extradata: *mut uint8_t, extradata_size: c_int) -> c_int;
	pub fn ff_vda_destroy_decoder(vda_ctx: *mut vda_context) -> c_int;

	pub fn av_vda_alloc_context() -> *mut AVVDAContext;
	pub fn av_vda_default_init(avctx: *mut AVCodecContext) -> c_int;
	pub fn av_vda_default_free(avctx: *mut AVCodecContext);
}
