use libc::{c_void, c_int, uint8_t};

pub type AVVorbisParseContext = c_void;

pub const VORBIS_FLAG_HEADER:  c_int = 0x00000001;
pub const VORBIS_FLAG_COMMENT: c_int = 0x00000002;
pub const VORBIS_FLAG_SETUP:   c_int = 0x00000004;

extern {
	pub fn av_vorbis_parse_init(extradata: *const uint8_t, extradata_size: c_int) -> *mut AVVorbisParseContext;
	pub fn av_vorbis_parse_free(s: *mut *mut AVVorbisParseContext);
	pub fn av_vorbis_parse_frame_flags(s: *mut AVVorbisParseContext, buf: *const uint8_t, buf_size: c_int, flags: *mut c_int) -> c_int;
	pub fn av_vorbis_parse_frame(s: *mut AVVorbisParseContext, buf: *const uint8_t, buf_size: c_int) -> c_int;
	pub fn av_vorbis_parse_reset(s: *mut AVVorbisParseContext);
}
