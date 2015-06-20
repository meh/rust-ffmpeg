use libc::{c_void, c_int, c_uint, uint8_t};

pub type AVSHA = c_void;

extern {
	pub static av_sha_size: c_int;

	pub fn av_sha_alloc() -> *mut AVSHA;
	pub fn av_sha_init(context: *mut AVSHA, bits: c_int) -> c_int;
	pub fn av_sha_update(context: *mut AVSHA, data: *const uint8_t, len: c_uint);
	pub fn av_sha_final(context: *mut AVSHA, digest: *mut uint8_t);
}
