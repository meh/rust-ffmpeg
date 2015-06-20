use libc::{c_void, c_int, c_uint, uint8_t};

pub type AVSHA512 = c_void;

extern {
	pub static av_sha512_size: c_int;

	pub fn av_sha512_alloc() -> *mut AVSHA512;
	pub fn av_sha512_init(context: *mut AVSHA512, bits: c_int) -> c_int;
	pub fn av_sha512_update(context: *mut AVSHA512, data: *const uint8_t, len: c_uint);
	pub fn av_sha512_final(context: *mut AVSHA512, digest: *mut uint8_t);
}
