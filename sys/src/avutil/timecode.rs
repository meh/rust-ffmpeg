use libc::{c_void, c_char, c_int, c_uint, uint32_t};
use super::rational::AVRational;

pub const AV_TIMECODE_FLAG_DROPFRAME:     uint32_t = 1 << 0;
pub const AV_TIMECODE_FLAG_24HOURSMAX:    uint32_t = 1 << 1;
pub const AV_TIMECODE_FLAG_ALLOWNEGATIVE: uint32_t = 1 << 2;

#[derive(Debug)]
#[repr(C)]
pub struct AVTimecode {
	pub start: c_int,
	pub flags: uint32_t,
	pub rate:  AVRational,
	pub fps:   c_uint,
}

extern {
	pub fn av_timecode_adjust_ntsc_framenum2(framenum: c_int, fps: c_int) -> c_int;
	pub fn av_timecode_get_smpte_from_framenum(tc: *const AVTimecode, framenum: c_int) -> uint32_t;
	pub fn av_timecode_make_string(tc: *const AVTimecode, buf: *mut c_char, framenum: c_int) -> *mut c_char;
	pub fn av_timecode_make_smpte_tc_string(buf: *mut c_char, tcsmpte: uint32_t, prevent_df: c_int) -> *mut c_char;
	pub fn av_timecode_make_mpeg_tc_string(buf: *mut c_char, tc25bit: uint32_t) -> *mut c_char;
	pub fn av_timecode_init(tc: *mut AVTimecode, rate: AVRational, flags: c_int, frame_start: c_int, log_ctx: *mut c_void) -> c_int;
	pub fn av_timecode_init_from_string(tc: *mut AVTimecode, rate: AVRational, string: *const c_char, log_ctx: *mut c_void) -> c_int;
	pub fn av_timecode_check_frame_rate(rate: AVRational) -> c_int;
}
