use libc::{c_int, uint8_t, uint32_t};

#[derive(Debug)]
#[repr(C)]
pub struct AVXTEA {
	key: [uint32_t; 16],
}

#[link(name = "avutil")]
extern {
	pub fn av_xtea_init(ctx: *mut AVXTEA, key: *const uint8_t);
	pub fn av_xtea_crypt(ctx: *mut AVXTEA, dst: *mut uint8_t, src: *const uint8_t, count: c_int, iv: *mut uint8_t, decrypt: c_int);
}
