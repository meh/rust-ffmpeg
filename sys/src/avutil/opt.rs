use libc::{c_void, c_char, c_int, c_uint, c_float, c_double, uint8_t, int64_t};
use super::rational::AVRational;
use super::dict::AVDictionary;
use super::log::AVClass;
use super::pixfmt::AVPixelFormat;
use super::samplefmt::AVSampleFormat;

pub const AV_OPT_FLAG_IMPLICIT_KEY:    c_int = 1;
pub const AV_OPT_FLAG_ENCODING_PARAM:  c_int = 1;
pub const AV_OPT_FLAG_DECODING_PARAM:  c_int = 2;
pub const AV_OPT_FLAG_AUDIO_PARAM:     c_int = 8;
pub const AV_OPT_FLAG_VIDEO_PARAM:     c_int = 16;
pub const AV_OPT_FLAG_SUBTITLE_PARAM:  c_int = 32;
pub const AV_OPT_FLAG_EXPORT:          c_int = 64;
pub const AV_OPT_FLAG_READONLY:        c_int = 128;
pub const AV_OPT_FLAG_FILTERING_PARAM: c_int = 1 << 16;

pub const AV_OPT_SEARCH_CHILDREN:           c_int = 0x0001;
pub const AV_OPT_SEARCH_FAKE_OBJ:           c_int = 0x0002;
pub const AV_OPT_MULTI_COMPONENT_RANGE:     c_int = 0x1000;
pub const AV_OPT_SERIALIZE_SKIP_DEFAULTS:   c_int = 0x00000001;
pub const AV_OPT_SERIALIZE_OPT_FLAGS_EXACT: c_int = 0x00000002;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVOptionType {
	AV_OPT_TYPE_FLAGS,
	AV_OPT_TYPE_INT,
	AV_OPT_TYPE_INT64,
	AV_OPT_TYPE_DOUBLE,
	AV_OPT_TYPE_FLOAT,
	AV_OPT_TYPE_STRING,
	AV_OPT_TYPE_RATIONAL,
	AV_OPT_TYPE_BINARY,
	AV_OPT_TYPE_DICT,
	AV_OPT_TYPE_CONST = 128,

	AV_OPT_TYPE_IMAGE_SIZE     = MKBETAG!(b'S', b'I', b'Z', b'E'),
	AV_OPT_TYPE_PIXEL_FMT      = MKBETAG!(b'P', b'F', b'M', b'T'),
	AV_OPT_TYPE_SAMPLE_FMT     = MKBETAG!(b'S', b'F', b'M', b'T'),
	AV_OPT_TYPE_VIDEO_RATE     = MKBETAG!(b'V', b'R', b'A', b'T'),
	AV_OPT_TYPE_DURATION       = MKBETAG!(b'D', b'U', b'R', b' '),
	AV_OPT_TYPE_COLOR          = MKBETAG!(b'C', b'O', b'L', b'R'),
	AV_OPT_TYPE_CHANNEL_LAYOUT = MKBETAG!(b'C', b'H', b'L', b'A'),
}

#[derive(Debug)]
#[repr(C)]
pub struct AVOption {
	pub name: *const c_char,
	pub help: *const c_char,

	pub offset: c_int,
	pub kind:   AVOptionType,

	pub default_val: [u8; 8],

	pub min: c_double,
	pub max: c_double,

	pub unit: *const c_char,
}

#[derive(Debug)]
#[repr(C)]
pub struct AVOptionRange {
	pub str: *const c_char,

	pub value_min: c_double,
	pub value_max: c_double,

	pub component_min: c_double,
	pub component_max: c_double,

	pub is_range: c_int,
}

#[derive(Debug)]
#[repr(C)]
pub struct AVOptionRanges {
	pub range: *mut *mut AVOptionRange,

	pub nb_ranges:     c_int,
	pub nb_components: c_int,
}

extern {
	pub fn av_opt_show2(obj: *mut c_void, av_log_obj: *mut c_void, req_flags: c_int, rej_flags: c_int) -> c_int;
	pub fn av_opt_set_defaults(s: *mut c_void);
	pub fn av_set_options_string(ctx: *mut c_void, opts: *const c_char, key_val_sep: *const c_char, pairs_sep: *const c_char) -> c_int;
	pub fn av_opt_set_from_string(ctx: *mut c_void, opts: *const c_char, shorthand: *const *const c_char, key_val_sep: *const c_char, pairs_sep: *const c_char) -> c_int;
	pub fn av_opt_free(obj: *mut c_void);
	pub fn av_opt_flag_is_set(obj: *mut c_void, field_name: *const c_char, flag_name: *const c_char) -> c_int;
	pub fn av_opt_set_dict(obj: *mut c_void, options: *mut *mut AVDictionary) -> c_int;
	pub fn av_opt_set_dict2(obj: *mut c_void, options: *mut *mut AVDictionary, search_flags: c_int) -> c_int;
	pub fn av_opt_get_key_value(ropts: *const *const c_char, key_val_sep: *const c_char, pairs_sep: *const c_char, flags: c_uint, rkey: *mut *mut c_char, rval: *mut *mut c_char) -> c_int;

	pub fn av_opt_eval_flags(obj: *mut c_void, o: *mut AVOption, val: *const c_char, flags_out: *mut c_int) -> c_int;
	pub fn av_opt_eval_int(obj: *mut c_void, o: *mut AVOption, val: *const c_char, int_out: *mut c_int) -> c_int;
	pub fn av_opt_eval_int64(obj: *mut c_void, o: *mut AVOption, val: *const c_char, int64_out: *mut int64_t) -> c_int;
	pub fn av_opt_eval_float(obj: *mut c_void, o: *mut AVOption, val: *const c_char, float_out: *mut c_float) -> c_int;
	pub fn av_opt_eval_double(obj: *mut c_void, o: *mut AVOption, val: *const c_char, double_out: *mut c_double) -> c_int;
	pub fn av_opt_eval_q(obj: *mut c_void, o: *mut AVOption, val: *const c_char, q_out: *mut AVRational) -> c_int;

	pub fn av_opt_find(obj: *mut c_void, name: *const c_char, unit: *const c_char, opt_flags: c_int, search_flags: c_int) -> *const AVOption;
	pub fn av_opt_find2(obj: *mut c_void, name: *const c_char, unit: *const c_char, opt_flags: c_int, search_flags: c_int, target_obj: *mut *mut c_void) -> *const AVOption;

	pub fn av_opt_next(obj: *mut c_void, prev: *const AVOption) -> *const AVOption;
	pub fn av_opt_child_next(obj: *mut c_void, prev: *mut c_void) -> *mut c_void;
	pub fn av_opt_child_class_next(parent: *const AVClass, prev: *const AVClass) -> *const AVClass;

	pub fn av_opt_set(obj: *mut c_void, name: *const c_char, val: *const c_char, search_flags: c_int) -> c_int;
	pub fn av_opt_set_int(obj: *mut c_void, name: *const c_char, val: int64_t, search_flags: c_int) -> c_int;
	pub fn av_opt_set_double(obj: *mut c_void, name: *const c_char, val: c_double, search_flags: c_int) -> c_int;
	pub fn av_opt_set_q(obj: *mut c_void, name: *const c_char, val: AVRational, search_flags: c_int) -> c_int;
	pub fn av_opt_set_bin(obj: *mut c_void, name: *const c_char, val: *const uint8_t, len: c_int, search_flags: c_int) -> c_int;
	pub fn av_opt_set_image_size(obj: *mut c_void, name: *const c_char, w: c_int, h: c_int, search_flags: c_int) -> c_int;
	pub fn av_opt_set_pixel_fmt(obj: *mut c_void, name: *const c_char, fmt: AVPixelFormat, search_flags: c_int) -> c_int;
	pub fn av_opt_set_sample_fmt(obj: *mut c_void, name: *const c_char, fmt: AVSampleFormat, search_flags: c_int) -> c_int;
	pub fn av_opt_set_video_rate(obj: *mut c_void, name: *const c_char, val: AVRational, search_flags: c_int) -> c_int;
	pub fn av_opt_set_channel_layout(obj: *mut c_void, name: *const c_char, ch_layout: int64_t, search_flags: c_int) -> c_int;
	pub fn av_opt_set_dict_val(obj: *mut c_void, name: *const c_char, val: *const AVDictionary, search_flags: c_int) -> c_int;

	pub fn av_opt_get(obj: *mut c_void, name: *const c_char, search_flags: c_int, out_val: *mut *mut uint8_t) -> c_int;
	pub fn av_opt_get_int(obj: *mut c_void, name: *const c_char, search_flags: c_int, out_val: *mut int64_t) -> c_int;
	pub fn av_opt_get_double(obj: *mut c_void, name: *const c_char, search_flags: c_int, out_val: *mut c_double) -> c_int;
	pub fn av_opt_get_image_size(obj: *mut c_void, name: *const c_char, search_flags: c_int, w_out: *mut c_int, h_out: *mut c_int) -> c_int;
	pub fn av_opt_get_pixel_fmt(obj: *mut c_void, name: *const c_char, search_flags: c_int, out_fmt: *mut AVPixelFormat) -> c_int;
	pub fn av_opt_get_sample_fmt(obj: *mut c_void, name: *const c_char, search_flags: c_int, out_fmt: *mut AVSampleFormat) -> c_int;
	pub fn av_opt_get_video_rate(obj: *mut c_void, name: *const c_char, search_flags: c_int, out_val: *mut AVRational) -> c_int;
	pub fn av_opt_get_channel_layout(obj: *mut c_void, name: *const c_char, search_flags: c_int, ch_layout: *mut int64_t) -> c_int;
	pub fn av_opt_get_dict_val(obj: *mut c_void, name: *const c_char, search_flags: c_int, out_val: *mut *mut AVDictionary) -> c_int;

	pub fn av_opt_ptr(avclass: *const AVClass, obj: *mut c_void, name: *const c_char);
	pub fn av_opt_freep_ranges(ranges: *mut *mut AVOptionRanges);
	pub fn av_opt_query_ranges(ranges: *mut *mut AVOptionRanges, obj: *mut c_void, key: *const c_char, flags: c_int) -> c_int;
	pub fn av_opt_copy(dest: *mut c_void, src: *mut c_void) -> c_int;
	pub fn av_opt_query_ranges_default(ranges: *mut *mut AVOptionRanges, obj: *mut c_void, key: *const c_char, flags: c_int) -> c_int;
	pub fn av_opt_is_set_to_default(obj: *mut c_void, o: *const AVOption) -> c_int;
	pub fn av_opt_is_set_to_default_by_name(obj: *mut c_void, name: *const c_char, search_flags: c_int) -> c_int;
	pub fn av_opt_serialize(obj: *mut c_void, opt_flags: c_int, flags: c_int, buffer: *mut *mut c_char, key_val_sep: c_char, pairs_sep: c_char) -> c_int;
}
