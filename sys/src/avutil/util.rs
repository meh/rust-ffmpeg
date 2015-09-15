use libc::{c_int, c_uint, c_char, int64_t};
use super::rational::AVRational;

pub const AV_NOPTS_VALUE: int64_t    = 0x8000000000000000u64 as int64_t;
pub const AV_TIME_BASE:   int64_t    = 1000000;
pub const AV_TIME_BASE_Q: AVRational = AVRational { num: 1, den: AV_TIME_BASE as c_int };

pub const FF_LAMBDA_SHIFT: c_int = 7;
pub const FF_LAMBDA_SCALE: c_int = 1 << FF_LAMBDA_SHIFT;
pub const FF_QP2LAMBDA:    c_int = 118;
pub const FF_LAMBDA_MAX:   c_int = 256 * 128 - 1;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVMediaType {
	AVMEDIA_TYPE_UNKNOWN = -1,
	AVMEDIA_TYPE_VIDEO,
	AVMEDIA_TYPE_AUDIO,
	AVMEDIA_TYPE_DATA,
	AVMEDIA_TYPE_SUBTITLE,
	AVMEDIA_TYPE_ATTACHMENT,
	AVMEDIA_TYPE_NB
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVPictureType {
	AV_PICTURE_TYPE_NONE = 0,
	AV_PICTURE_TYPE_I,
	AV_PICTURE_TYPE_P,
	AV_PICTURE_TYPE_B,
	AV_PICTURE_TYPE_S,
	AV_PICTURE_TYPE_SI,
	AV_PICTURE_TYPE_SP,
	AV_PICTURE_TYPE_BI,
}

extern {
	pub fn av_version_info() -> *const c_char;

	pub fn avutil_version() -> c_uint;
	pub fn avutil_configuration() -> *const c_char;
	pub fn avutil_license() -> *const c_char;

	pub fn av_get_time_base_q() -> AVRational;

	pub fn av_get_media_type_string(media_type: AVMediaType) -> *const c_char;
	pub fn av_get_picture_type_char(pict_type: AVPictureType) -> c_char;
}
