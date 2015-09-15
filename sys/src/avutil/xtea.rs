use libc::{c_int, uint8_t, uint32_t};

#[derive(Debug)]
#[repr(C)]
pub struct AVXTEA {
	pub key: [uint32_t; 16],
}

extern {
	pub fn av_xtea_alloc() -> *mut AVXTEA;
	pub fn av_xtea_init(ctx: *mut AVXTEA, key: *const uint8_t);
	pub fn av_xtea_crypt(ctx: *mut AVXTEA, dst: *mut uint8_t, src: *const uint8_t, count: c_int, iv: *mut uint8_t, decrypt: c_int);
}
