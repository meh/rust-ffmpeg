use libc::{c_char, c_int, c_uint, uint8_t, int32_t, size_t, c_double};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVEscapeMode {
	AV_ESCAPE_MODE_AUTO,
	AV_ESCAPE_MODE_BACKSLASH,
	AV_ESCAPE_MODE_QUOTE,
}

pub const AV_ESCAPE_FLAG_WHITESPACE: c_int = 0x01;
pub const AV_ESCAPE_FLAG_STRICT:     c_int = 0x02;

pub const AV_UTF8_FLAG_ACCEPT_INVALID_BIG_CODES:          c_uint = 1;
pub const AV_UTF8_FLAG_ACCEPT_NON_CHARACTERS:             c_uint = 2;
pub const AV_UTF8_FLAG_ACCEPT_SURROGATES:                 c_uint = 4;
pub const AV_UTF8_FLAG_EXCLUDE_XML_INVALID_CONTROL_CODES: c_uint = 8;
pub const AV_UTF8_FLAG_ACCEPT_ALL:                        c_uint = AV_UTF8_FLAG_ACCEPT_INVALID_BIG_CODES | AV_UTF8_FLAG_ACCEPT_NON_CHARACTERS | AV_UTF8_FLAG_ACCEPT_SURROGATES;


#[inline(always)]
pub unsafe fn av_strnlen(s: *const c_char, len: size_t) -> size_t {
	let mut result = 0;
	let mut ptr    = s;

	while result < len && *ptr != 0 {
		ptr     = ptr.offset(1);
		result += 1;
	}

	result
}

#[inline(always)]
pub unsafe fn av_toupper(c: c_int) -> c_int {
	if c >= b'a' as c_int && c <= b'z' as c_int {
		c ^ 0x20
	}
	else {
		c
	}
}

#[inline(always)]
pub unsafe fn av_tolower(c: c_int) -> c_int {
	if c >= b'A' as c_int && c <= b'Z' as c_int {
		c ^ 0x20
	}
	else {
		c
	}
}

extern {
	pub fn av_strstart(string: *const c_char, pfx: *const c_char, ptr: *const *const c_char) -> c_int;
	pub fn av_stristart(string: *const c_char, pfx: *const c_char, ptr: *const *const c_char) -> c_int;
	pub fn av_stristr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
	pub fn av_strnstr(haystack: *const c_char, needle: *const c_char, hay_length: size_t) -> *mut c_char;
	pub fn av_strlcpy(dst: *mut c_char, src: *const c_char, size: size_t) -> size_t;
	pub fn av_strlcat(dst: *mut c_char, src: *const c_char, size: size_t) -> size_t;
	pub fn av_strlcatf(dst: *mut c_char, size: size_t, fmt: *const c_char, ...) -> size_t;
	pub fn av_asprintf(fmt: *const c_char, ...) -> *mut c_char;
	pub fn av_d2str(d: c_double) -> *mut c_char;
	pub fn av_get_token(buf: *const *const c_char, term: *const c_char) -> *mut c_char;
	pub fn av_strtok(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;

	pub fn av_isdigit(c: c_int) -> c_int;
	pub fn av_isgraph(c: c_int) -> c_int;
	pub fn av_isspace(c: c_int) -> c_int;
	pub fn av_isxdigit(c: c_int) -> c_int;

	pub fn av_strcasecmp(a: *const c_char, b: *const c_char) -> c_int;
	pub fn av_strncasecmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;

	pub fn av_basename(path: *const c_char) -> *const c_char;
	pub fn av_dirname(path: *mut c_char) -> *const c_char;

	pub fn av_match_name(name: *const c_char, names: *const c_char) -> c_int;
	pub fn av_append_path_component(path: *const c_char, component: *const c_char) -> *mut c_char;
	pub fn av_escape(dst: *mut *mut c_char, src: *const c_char, special_chars: *const c_char, mode: AVEscapeMode, flags: c_int) -> c_int;
	pub fn av_utf8_decode(codep: *mut int32_t, bufp: *const *const uint8_t, buf_end: *const uint8_t, flags: c_uint) -> c_int;
	pub fn av_match_list(name: *const c_char, list: *const c_char, separator: c_char) -> c_int;
}
