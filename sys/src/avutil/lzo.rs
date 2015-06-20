use libc::{c_void, c_int};

pub const AV_LZO_INPUT_DEPLETED:  c_int = 1;
pub const AV_LZO_OUTPUT_FULL:     c_int = 2;
pub const AV_LZO_INVALID_BACKPTR: c_int = 4;
pub const AV_LZO_ERROR:           c_int = 8;
pub const AV_LZO_INPUT_PADDING:   c_int = 8;
pub const AV_LZO_OUTPUT_PADDING:  c_int = 12;

extern {
	pub fn av_lzo1x_decode(out: *mut c_void, outlen: *mut c_int, inp: *const c_void, inlen: *mut c_int) -> c_int;
}
