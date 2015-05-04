use libc::{c_void, c_int, c_uint, uint8_t};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVHMACType {
	AV_HMAC_MD5,
	AV_HMAC_SHA1,
	AV_HMAC_SHA224 = 10,
	AV_HMAC_SHA256,
	AV_HMAC_SHA384,
	AV_HMAC_SHA512,
}

pub type AVHMAC = c_void;

#[link(name = "avutil")]
extern {
	pub fn av_hmac_alloc(kind: AVHMACType) -> *mut AVHMAC;
	pub fn av_hmac_free(ctx: *mut AVHMAC);
	pub fn av_hmac_init(ctx: *mut AVHMAC, key: *const uint8_t, keylen: c_uint);
	pub fn av_hmac_update(ctx: *mut AVHMAC, data: *const uint8_t, len: c_uint);
	pub fn av_hmac_final(ctx: *mut AVHMAC, out: *mut uint8_t, outlen: c_uint) -> c_int;
	pub fn av_hmac_calc(ctx: *mut AVHMAC, data: *const uint8_t, len: c_uint, key: *const uint8_t, keylen: c_uint, out: *mut uint8_t, outlen: c_uint) -> c_int;
}
