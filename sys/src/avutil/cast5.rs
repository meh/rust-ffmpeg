use libc::{c_void, c_int, uint8_t};

pub type AVCAST5 = c_void;

extern {
	pub static av_cast5_size: c_int;

	pub fn av_cast5_alloc() -> *mut AVCAST5;
	pub fn av_cast5_init(ctx: *mut AVCAST5, key: *const uint8_t, key_bits: c_int) -> c_int;
	pub fn av_cast5_crypt(ctx: *mut AVCAST5, dst: *mut uint8_t, src: *const uint8_t, count: c_int, decrypt: c_int);
	pub fn av_cast5_crypt2(ctx: *mut AVCAST5, dst: *mut uint8_t, src: *const uint8_t, count: c_int, iv: *mut uint8_t, decrypt: c_int);
}
