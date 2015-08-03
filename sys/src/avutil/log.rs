use libc::{c_void, c_char, c_int};
use super::opt::{AVOption, AVOptionRanges};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVClassCategory {
	AV_CLASS_CATEGORY_NA = 0,
	AV_CLASS_CATEGORY_INPUT,
	AV_CLASS_CATEGORY_OUTPUT,
	AV_CLASS_CATEGORY_MUXER,
	AV_CLASS_CATEGORY_DEMUXER,
	AV_CLASS_CATEGORY_ENCODER,
	AV_CLASS_CATEGORY_DECODER,
	AV_CLASS_CATEGORY_FILTER,
	AV_CLASS_CATEGORY_BITSTREAM_FILTER,
	AV_CLASS_CATEGORY_SWSCALER,
	AV_CLASS_CATEGORY_SWRESAMPLER,
	AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT = 40,
	AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT,
	AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT,
	AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT,
	AV_CLASS_CATEGORY_DEVICE_OUTPUT,
	AV_CLASS_CATEGORY_DEVICE_INPUT,
	AV_CLASS_CATEGORY_NB,
}

#[inline(always)]
pub fn AV_IS_INPUT_DEVICE(category: AVClassCategory) -> bool {
	category == AVClassCategory::AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT ||
	category == AVClassCategory::AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT ||
	category == AVClassCategory::AV_CLASS_CATEGORY_DEVICE_INPUT
}

#[inline(always)]
pub fn AV_IS_OUTPUT_DEVICE(category: AVClassCategory) -> bool {
	category == AVClassCategory::AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT ||
	category == AVClassCategory::AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT ||
	category == AVClassCategory::AV_CLASS_CATEGORY_DEVICE_OUTPUT
}

#[repr(C)]
pub struct AVClass {
	pub class_name: *const c_char,
	pub item_name:  extern fn(ctx: *mut c_void) -> *const c_char,

	pub option:  *const AVOption,
	pub version: c_int,

	pub log_level_offset_offset:   c_int,
	pub parent_log_context_offset: c_int,

	pub child_next:       extern fn(obj: *mut c_void, prev: *mut c_void) -> *mut c_void,
	pub child_class_next: extern fn(prev: *const AVClass) -> *const AVClass,

	pub category: AVClassCategory,

	pub get_category: extern fn(ctx: *mut c_void) -> AVClassCategory,
	pub query_ranges: extern fn(ranges: *mut *mut AVOptionRanges, obj: *mut c_void, key: *const c_char, flags: c_int) -> c_int,
}

pub const AV_LOG_QUIET:      c_int = -8;
pub const AV_LOG_PANIC:      c_int = 0;
pub const AV_LOG_FATAL:      c_int = 8;
pub const AV_LOG_ERROR:      c_int = 16;
pub const AV_LOG_WARNING:    c_int = 24;
pub const AV_LOG_INFO:       c_int = 32;
pub const AV_LOG_VERBOSE:    c_int = 40;
pub const AV_LOG_DEBUG:      c_int = 48;
pub const AV_LOG_TRACE:      c_int = 56;
pub const AV_LOG_MAX_OFFSET: c_int = (AV_LOG_DEBUG - AV_LOG_QUIET);

pub const AV_LOG_SKIP_REPEATED: c_int = 1;
pub const AV_LOG_PRINT_LEVEL:   c_int = 2;

#[inline(always)]
pub fn AV_LOG_C(x: c_int) -> c_int {
	x << 8
}

extern {
	pub fn av_log(avcl: *mut c_void, level: c_int, fmt: *const c_char, ...);
	//pub fn av_vlog(avcl: *mut c_void, level: c_int, fmt: *const c_char, vl: va_list);
	pub fn av_log_get_level() -> c_int;
	pub fn av_log_set_level(level: c_int);
	//pub fn av_log_set_callback(callback: extern fn(*mut c_void, c_int, *const c_char, va_list));
	//pub fn av_log_default_callback(avcl: *mut c_void, level: c_int, fmt: *const c_char, vl: va_list);
	pub fn av_default_item_name(ctx: *mut c_void) -> *const c_char;
	pub fn av_default_get_category(ptr: *mut c_void) -> AVClassCategory;
	//pub fn av_log_format_line(ptr: *mut c_void, level: c_int, fmt: *const c_char, vl: va_list, line: *mut c_char, line_size: c_int, print_prefix: *mut c_int);
	pub fn av_log_set_flags(arg: c_int);
	pub fn av_log_get_flags() -> c_int;
}
