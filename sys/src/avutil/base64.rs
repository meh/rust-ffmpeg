use libc::{c_char, c_int, uint8_t, size_t};

#[inline(always)]
pub fn AV_BASE64_SIZE(x: size_t) -> size_t {
	(x + 2) / 3 * 4 + 1
}

extern {
	pub fn av_base64_decode(out: *mut uint8_t, inp: *const c_char, out_size: c_int) -> c_int;
	pub fn av_base64_encode(out: *mut c_char, out_size: c_int, inp: *const uint8_t, in_size: c_int) -> *mut c_char;
}
