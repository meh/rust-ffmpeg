use libc::{c_void, c_int, c_char, int64_t};

pub const AV_DICT_MATCH_CASE:      c_int = 1;
pub const AV_DICT_IGNORE_SUFFIX:   c_int = 2;
pub const AV_DICT_DONT_STRDUP_KEY: c_int = 4;
pub const AV_DICT_DONT_STRDUP_VAL: c_int = 8;
pub const AV_DICT_DONT_OVERWRITE:  c_int = 16;
pub const AV_DICT_APPEND:          c_int = 32;

#[repr(C)]
pub struct AVDictionaryEntry {
	pub key:   *mut c_char,
	pub value: *mut c_char,
}

pub type AVDictionary = c_void;

extern {
	pub fn av_dict_get(m: *const AVDictionary, key: *const c_char, prev: *const AVDictionaryEntry, flags: c_int) -> *mut AVDictionaryEntry;
	pub fn av_dict_count(m: *const AVDictionary) -> c_int;
	pub fn av_dict_set(pm: *mut *mut AVDictionary, key: *const c_char, value: *const c_char, flags: c_int) -> c_int;
	pub fn av_dict_set_int(pm: *mut *mut AVDictionary, key: *const c_char, value: int64_t, flags: c_int) -> c_int;
	pub fn av_dict_parse_string(pm: *mut *mut AVDictionary, string: *const c_char, key_val_sep: *const c_char, pairs_sep: *const c_char, flags: c_int) -> c_int;
	pub fn av_dict_copy(dst: *mut *mut AVDictionary, src: *const AVDictionary, flags: c_int);
	pub fn av_dict_free(m: *mut *mut AVDictionary);
	pub fn av_dict_get_string(m: *const AVDictionary, buffer: *mut *mut c_char, key_val_sep: c_char, pairs_sep: c_char) -> c_int;
}
