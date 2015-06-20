use libc::{c_void, c_int, uint8_t};

pub type AVAES = c_void;

extern {
	pub static av_aes_size: c_int;

	pub fn av_aes_alloc() -> *mut AVAES;
	pub fn av_aes_init(a: *mut AVAES, key: *const uint8_t, key_bits: c_int, decrypt: c_int) -> c_int;
	pub fn av_aes_crypt(a: *mut AVAES, dst: *mut uint8_t, src: *const uint8_t, count: c_int, iv: *mut uint8_t, decrypt: c_int);
}
