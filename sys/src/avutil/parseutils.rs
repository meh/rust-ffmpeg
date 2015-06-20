use libc::{c_void, c_char, c_int, uint8_t, int64_t, time_t};
use std::ptr;
use super::rational::AVRational;
use super::log::AV_LOG_MAX_OFFSET;

#[inline(always)]
pub unsafe fn av_parse_ratio_quiet(q: *mut AVRational, string: *const c_char, max: c_int) -> c_int {
	av_parse_ratio(q, string, max, AV_LOG_MAX_OFFSET, ptr::null_mut())
}

extern {
	pub fn av_parse_ratio(q: *mut AVRational, string: *const c_char, max: c_int, log_offset: c_int, log_ctx: *mut c_void) -> c_int;
	pub fn av_parse_video_size(width_ptr: *mut c_int, height_ptr: *mut c_int, string: *const c_char) -> c_int;
	pub fn av_parse_video_rate(rate: *mut AVRational, string: *const c_char) -> c_int;
	pub fn av_parse_color(rgba_color: *mut uint8_t, color_string: *const c_char, slen: c_int, log_ctx: *mut c_void) -> c_int;
	pub fn av_get_known_color_name(color_idx: c_int, rgb: *const *const uint8_t) -> *const c_char;
	pub fn av_parse_time(timeval: *mut int64_t, timestr: *const c_char, duration: c_int) -> c_int;
	pub fn av_find_info_tag(arg: *mut c_char, arg_size: c_int, tag1: *const c_char, info: *const c_char) -> c_int;
	pub fn av_small_strptime(p: *const c_char, fmt: *const c_char, dt: *mut c_void) -> *mut c_char;
	pub fn av_timegm(tm: *mut c_void) -> time_t;
}
