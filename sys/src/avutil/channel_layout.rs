use libc::{c_char, c_int, c_uint, c_ulonglong, uint64_t, int64_t};
use super::bprint::AVBPrint;

pub const AV_CH_FRONT_LEFT:            c_ulonglong = 0x00000001;
pub const AV_CH_FRONT_RIGHT:           c_ulonglong = 0x00000002;
pub const AV_CH_FRONT_CENTER:          c_ulonglong = 0x00000004;
pub const AV_CH_LOW_FREQUENCY:         c_ulonglong = 0x00000008;
pub const AV_CH_BACK_LEFT:             c_ulonglong = 0x00000010;
pub const AV_CH_BACK_RIGHT:            c_ulonglong = 0x00000020;
pub const AV_CH_FRONT_LEFT_OF_CENTER:  c_ulonglong = 0x00000040;
pub const AV_CH_FRONT_RIGHT_OF_CENTER: c_ulonglong = 0x00000080;
pub const AV_CH_BACK_CENTER:           c_ulonglong = 0x00000100;
pub const AV_CH_SIDE_LEFT:             c_ulonglong = 0x00000200;
pub const AV_CH_SIDE_RIGHT:            c_ulonglong = 0x00000400;
pub const AV_CH_TOP_CENTER:            c_ulonglong = 0x00000800;
pub const AV_CH_TOP_FRONT_LEFT:        c_ulonglong = 0x00001000;
pub const AV_CH_TOP_FRONT_CENTER:      c_ulonglong = 0x00002000;
pub const AV_CH_TOP_FRONT_RIGHT:       c_ulonglong = 0x00004000;
pub const AV_CH_TOP_BACK_LEFT:         c_ulonglong = 0x00008000;
pub const AV_CH_TOP_BACK_CENTER:       c_ulonglong = 0x00010000;
pub const AV_CH_TOP_BACK_RIGHT:        c_ulonglong = 0x00020000;
pub const AV_CH_STEREO_LEFT:           c_ulonglong = 0x20000000;
pub const AV_CH_STEREO_RIGHT:          c_ulonglong = 0x40000000;
pub const AV_CH_WIDE_LEFT:             c_ulonglong = 0x0000000080000000;
pub const AV_CH_WIDE_RIGHT:            c_ulonglong = 0x0000000100000000;
pub const AV_CH_SURROUND_DIRECT_LEFT:  c_ulonglong = 0x0000000200000000;
pub const AV_CH_SURROUND_DIRECT_RIGHT: c_ulonglong = 0x0000000400000000;
pub const AV_CH_LOW_FREQUENCY_2:       c_ulonglong = 0x0000000800000000;

pub const AV_CH_LAYOUT_NATIVE: c_ulonglong = 0x8000000000000000;

pub const AV_CH_LAYOUT_MONO:              c_ulonglong = AV_CH_FRONT_CENTER;
pub const AV_CH_LAYOUT_STEREO:            c_ulonglong = AV_CH_FRONT_LEFT | AV_CH_FRONT_RIGHT;
pub const AV_CH_LAYOUT_2POINT1:           c_ulonglong = AV_CH_LAYOUT_STEREO | AV_CH_LOW_FREQUENCY;
pub const AV_CH_LAYOUT_2_1:               c_ulonglong = AV_CH_LAYOUT_STEREO | AV_CH_BACK_CENTER;
pub const AV_CH_LAYOUT_SURROUND:          c_ulonglong = AV_CH_LAYOUT_STEREO | AV_CH_FRONT_CENTER;
pub const AV_CH_LAYOUT_3POINT1:           c_ulonglong = AV_CH_LAYOUT_SURROUND | AV_CH_LOW_FREQUENCY;
pub const AV_CH_LAYOUT_4POINT0:           c_ulonglong = AV_CH_LAYOUT_SURROUND | AV_CH_BACK_CENTER;
pub const AV_CH_LAYOUT_4POINT1:           c_ulonglong = AV_CH_LAYOUT_4POINT0 | AV_CH_LOW_FREQUENCY;
pub const AV_CH_LAYOUT_2_2:               c_ulonglong = AV_CH_LAYOUT_STEREO | AV_CH_SIDE_LEFT | AV_CH_SIDE_RIGHT;
pub const AV_CH_LAYOUT_QUAD:              c_ulonglong = AV_CH_LAYOUT_STEREO | AV_CH_BACK_LEFT | AV_CH_BACK_RIGHT;
pub const AV_CH_LAYOUT_5POINT0:           c_ulonglong = AV_CH_LAYOUT_SURROUND | AV_CH_SIDE_LEFT | AV_CH_SIDE_RIGHT;
pub const AV_CH_LAYOUT_5POINT1:           c_ulonglong = AV_CH_LAYOUT_5POINT0 | AV_CH_LOW_FREQUENCY;
pub const AV_CH_LAYOUT_5POINT0_BACK:      c_ulonglong = AV_CH_LAYOUT_SURROUND | AV_CH_BACK_LEFT | AV_CH_BACK_RIGHT;
pub const AV_CH_LAYOUT_5POINT1_BACK:      c_ulonglong = AV_CH_LAYOUT_5POINT0_BACK | AV_CH_LOW_FREQUENCY;
pub const AV_CH_LAYOUT_6POINT0:           c_ulonglong = AV_CH_LAYOUT_5POINT0 | AV_CH_BACK_CENTER;
pub const AV_CH_LAYOUT_6POINT0_FRONT:     c_ulonglong = AV_CH_LAYOUT_2_2 | AV_CH_FRONT_LEFT_OF_CENTER | AV_CH_FRONT_RIGHT_OF_CENTER;
pub const AV_CH_LAYOUT_HEXAGONAL:         c_ulonglong = AV_CH_LAYOUT_5POINT0_BACK | AV_CH_BACK_CENTER;
pub const AV_CH_LAYOUT_6POINT1:           c_ulonglong = AV_CH_LAYOUT_5POINT1 | AV_CH_BACK_CENTER;
pub const AV_CH_LAYOUT_6POINT1_BACK:      c_ulonglong = AV_CH_LAYOUT_5POINT1_BACK | AV_CH_BACK_CENTER;
pub const AV_CH_LAYOUT_6POINT1_FRONT:     c_ulonglong = AV_CH_LAYOUT_6POINT0_FRONT | AV_CH_LOW_FREQUENCY;
pub const AV_CH_LAYOUT_7POINT0:           c_ulonglong = AV_CH_LAYOUT_5POINT0 | AV_CH_BACK_LEFT | AV_CH_BACK_RIGHT;
pub const AV_CH_LAYOUT_7POINT0_FRONT:     c_ulonglong = AV_CH_LAYOUT_5POINT0 | AV_CH_FRONT_LEFT_OF_CENTER | AV_CH_FRONT_RIGHT_OF_CENTER;
pub const AV_CH_LAYOUT_7POINT1:           c_ulonglong = AV_CH_LAYOUT_5POINT1 | AV_CH_BACK_LEFT | AV_CH_BACK_RIGHT;
pub const AV_CH_LAYOUT_7POINT1_WIDE:      c_ulonglong = AV_CH_LAYOUT_5POINT1 | AV_CH_FRONT_LEFT_OF_CENTER | AV_CH_FRONT_RIGHT_OF_CENTER;
pub const AV_CH_LAYOUT_7POINT1_WIDE_BACK: c_ulonglong = AV_CH_LAYOUT_5POINT1_BACK | AV_CH_FRONT_LEFT_OF_CENTER | AV_CH_FRONT_RIGHT_OF_CENTER;
pub const AV_CH_LAYOUT_OCTAGONAL:         c_ulonglong = AV_CH_LAYOUT_5POINT0 | AV_CH_BACK_LEFT | AV_CH_BACK_CENTER | AV_CH_BACK_RIGHT;
pub const AV_CH_LAYOUT_HEXADECAGONAL:     c_ulonglong = AV_CH_LAYOUT_OCTAGONAL | AV_CH_WIDE_LEFT | AV_CH_WIDE_RIGHT | AV_CH_TOP_BACK_LEFT | AV_CH_TOP_BACK_RIGHT | AV_CH_TOP_BACK_CENTER | AV_CH_TOP_FRONT_CENTER | AV_CH_TOP_FRONT_LEFT | AV_CH_TOP_FRONT_RIGHT;
pub const AV_CH_LAYOUT_STEREO_DOWNMIX:    c_ulonglong = AV_CH_STEREO_LEFT | AV_CH_STEREO_RIGHT;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVMatrixEncoding {
	AV_MATRIX_ENCODING_NONE,
	AV_MATRIX_ENCODING_DOLBY,
	AV_MATRIX_ENCODING_DPLII,
	AV_MATRIX_ENCODING_DPLIIX,
	AV_MATRIX_ENCODING_DPLIIZ,
	AV_MATRIX_ENCODING_DOLBYEX,
	AV_MATRIX_ENCODING_DOLBYHEADPHONE,
	AV_MATRIX_ENCODING_NB
}

extern {
	pub fn av_get_channel_layout(name: *const c_char) -> uint64_t;
	pub fn av_get_channel_layout_string(buf: *mut c_char, buf_size: c_int, nb_channels: c_int, channel_layout: uint64_t);
	pub fn av_bprint_channel_layout(bp: *mut AVBPrint, nb_channels: c_int, channel_layout: uint64_t);
	pub fn av_get_channel_layout_nb_channels(channel_layout: uint64_t) -> c_int;
	pub fn av_get_default_channel_layout(nb_channels: c_int) -> int64_t;
	pub fn av_get_channel_layout_channel_index(channel_layout: uint64_t, channel: uint64_t) -> c_int;
	pub fn av_channel_layout_extract_channel(channel_layout: uint64_t, index: c_int) -> uint64_t;
	pub fn av_get_channel_name(channel: uint64_t) -> *const c_char;
	pub fn av_get_channel_description(channel: uint64_t) -> *const c_char;
	pub fn av_get_standard_channel_layout(index: c_uint, layout: *mut uint64_t, name: *const *const c_char) -> c_int;
}
