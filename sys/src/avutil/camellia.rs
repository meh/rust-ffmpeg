use libc::{c_void, c_int, uint8_t};

pub type AVCAMELLIA = c_void;

extern {
	pub static av_camellia_size: c_int;

	pub fn av_camellia_alloc() -> *mut AVCAMELLIA;
	pub fn av_camellia_init(ctx: *mut AVCAMELLIA, key: *const uint8_t, key_bits: c_int) -> c_int;
	pub fn av_camellia_crypt(ctx: *mut AVCAMELLIA, dst: *mut uint8_t, src: *const uint8_t, count: c_int, iv: *mut uint8_t, decrypt: c_int);
}
