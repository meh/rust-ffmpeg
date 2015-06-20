use libc::{c_void, c_char, c_uchar, c_uint, c_int};
use super::string::AVEscapeMode;

#[repr(C)]
pub struct AVBPrint {
	pub string:   *mut c_char,
	pub len:      c_uint,
	pub size:     c_uint,
	pub size_max: c_uint,

	buffer: [c_char; 1024],
}

#[inline(always)]
pub unsafe fn av_bprint_is_complete(buf: *const AVBPrint) -> c_int {
	if (*buf).len < (*buf).size {
		1
	}
	else {
		0
	}
}

extern {
	pub fn av_bprint_init(buf: *mut AVBPrint, size_init: c_uint, size_max: c_uint);
	pub fn av_bprint_init_for_buffer(buf: *mut AVBPrint, buffer: *mut c_char, size: c_uint);
	pub fn av_bprintf(buf: *mut AVBPrint, fmt: *const c_char, ...);
	//pub fn av_vbprintf(buf: *mut AVBPrint, fmt: *const c_char, vl_arg: va_list);
	pub fn av_bprint_chars(buf: *mut AVBPrint, c: c_char, n: c_uint);
	pub fn av_bprint_append_data(buf: *mut AVBPrint, data: *const c_char, size: c_uint);
	pub fn av_bprint_strftime(buf: *mut AVBPrint, fmt: *const c_char, tm: *const c_void);
	pub fn av_bprint_get_buffer(buf: *mut AVBPrint, size: c_uint, mem: *mut *mut c_uchar, actual_size: *mut c_uint);
	pub fn av_bprint_clear(buf: *mut AVBPrint);
	pub fn av_bprint_finalize(buf: *mut AVBPrint, ret_str: *mut *mut c_char) -> c_int;
	pub fn av_bprint_escape(dstbuf: *mut AVBPrint, src: *const c_char, special_chars: *const c_char, mode: AVEscapeMode, flags: c_int);
}
