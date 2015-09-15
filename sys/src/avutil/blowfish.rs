use libc::{c_int, uint8_t, uint32_t};

pub const AV_BF_ROUNDS: c_int = 16;

#[repr(C)]
pub struct AVBlowfish {
	pub p: [uint32_t; AV_BF_ROUNDS as usize + 2],
	pub s: [[uint32_t; 256]; 4],
}

extern {
	pub fn av_blowfish_alloc() -> *mut AVBlowfish;
	pub fn av_blowfish_init(ctx: *mut AVBlowfish, key: *const uint8_t, key_len: c_int);
	pub fn av_blowfish_crypt_ecb(ctx: *mut AVBlowfish, xl: *mut uint32_t, xr: *mut uint32_t, decrypt: c_int);
	pub fn av_blowfish_crypt(ctx: *mut AVBlowfish, dst: *mut uint8_t, src: *const uint8_t, count: c_int, iv: *mut uint8_t, decrypt: c_int);
}
