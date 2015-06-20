use libc::{c_int, c_ulong, uint8_t};

extern {
	pub fn av_adler32_update(adler: c_ulong, buf: *const uint8_t, len: c_int) -> c_ulong;
}
