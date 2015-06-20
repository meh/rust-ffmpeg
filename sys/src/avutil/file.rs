use libc::{c_void, c_char, c_int, uint8_t, size_t};

extern {
	pub fn av_file_map(filename: *const c_char, bufptr: *mut *mut uint8_t, size: *mut size_t, log_offset: c_int, log_ctx: *mut c_void) -> c_int;
	pub fn av_file_unmap(bufptr: *mut uint8_t, size: size_t);
	pub fn av_tempfile(prefix: *const c_char, filename: *mut *mut c_char, log_offset: c_int, log_ctx: *mut c_void) -> c_int;
}
