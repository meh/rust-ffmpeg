use libc::{c_void, c_int, c_uint, uint8_t};

pub type AVRIPEMD = c_void;

extern {
	pub static av_ripemd_size: c_int;

	pub fn av_ripemd_init(context: *mut AVRIPEMD, bits: c_int) -> c_int;
	pub fn av_ripemd_update(context: *mut AVRIPEMD, data: *const uint8_t, len: c_uint);
	pub fn av_ripemd_final(context: *mut AVRIPEMD, digest: *mut uint8_t);
}
