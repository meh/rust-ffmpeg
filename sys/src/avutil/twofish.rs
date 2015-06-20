use libc::{c_void, c_int, uint8_t};

pub type AVTWOFISH = c_void;

extern {
	pub static av_twofish_size: c_int;

	pub fn av_twofish_alloc() -> *mut AVTWOFISH;
	pub fn av_twofish_init(ctx: *mut AVTWOFISH, key: *const uint8_t, key_bits: c_int) -> c_int;
	pub fn av_twofish_crypt(ctx: *mut AVTWOFISH, dst: *mut uint8_t, src: *const uint8_t, count: c_int, iv: *mut uint8_t, decrypt: c_int);
}
