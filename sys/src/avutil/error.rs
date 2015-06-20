use libc::{c_char, c_int, size_t};

#[inline(always)]
pub fn AVERROR(e: c_int) -> c_int {
	-e
}

#[inline(always)]
pub fn AVUNERROR(e: c_int) -> c_int {
	-e
}

macro_rules! FFERRTAG {
	($a:expr, $b:expr, $c:expr, $d:expr) =>
		(-MKTAG!($a, $b, $c, $d) as c_int)
}

pub const AVERROR_BSF_NOT_FOUND:      c_int = FFERRTAG!(0xF8, b'B', b'S', b'F');
pub const AVERROR_BUG:                c_int = FFERRTAG!(b'B', b'U', b'G', b'!');
pub const AVERROR_BUFFER_TOO_SMALL:   c_int = FFERRTAG!(b'B', b'U', b'F', b'S');
pub const AVERROR_DECODER_NOT_FOUND:  c_int = FFERRTAG!(0xF8, b'D', b'E', b'C');
pub const AVERROR_DEMUXER_NOT_FOUND:  c_int = FFERRTAG!(0xF8, b'D', b'E', b'M');
pub const AVERROR_ENCODER_NOT_FOUND:  c_int = FFERRTAG!(0xF8, b'E', b'N', b'C');
pub const AVERROR_EOF:                c_int = FFERRTAG!(b'E', b'O', b'F', b' ');
pub const AVERROR_EXIT:               c_int = FFERRTAG!(b'E', b'X', b'I', b'T');
pub const AVERROR_EXTERNAL:           c_int = FFERRTAG!(b'E', b'X', b'T', b' ');
pub const AVERROR_FILTER_NOT_FOUND:   c_int = FFERRTAG!(0xF8, b'F', b'I', b'L');
pub const AVERROR_INVALIDDATA:        c_int = FFERRTAG!(b'I', b'N', b'D', b'A');
pub const AVERROR_MUXER_NOT_FOUND:    c_int = FFERRTAG!(0xF8, b'M', b'U', b'X');
pub const AVERROR_OPTION_NOT_FOUND:   c_int = FFERRTAG!(0xF8, b'O', b'P', b'T');
pub const AVERROR_PATCHWELCOME:       c_int = FFERRTAG!(b'P', b'A', b'W', b'E');
pub const AVERROR_PROTOCOL_NOT_FOUND: c_int = FFERRTAG!(0xF8, b'P', b'R', b'O');

pub const AVERROR_STREAM_NOT_FOUND: c_int = FFERRTAG!(0xF8, b'S', b'T', b'R');

pub const AVERROR_BUG2:           c_int = FFERRTAG!(b'B', b'U', b'G', b' ');
pub const AVERROR_UNKNOWN:        c_int = FFERRTAG!(b'U', b'N', b'K', b'N');
pub const AVERROR_EXPERIMENTAL:   c_int = -0x2bb2afa8;
pub const AVERROR_INPUT_CHANGED:  c_int = -0x636e6701;
pub const AVERROR_OUTPUT_CHANGED: c_int = -0x636e6702;

pub const AVERROR_HTTP_BAD_REQUEST:  c_int = FFERRTAG!(0xF8, b'4', b'0', b'0');
pub const AVERROR_HTTP_UNAUTHORIZED: c_int = FFERRTAG!(0xF8, b'4', b'0', b'1');
pub const AVERROR_HTTP_FORBIDDEN:    c_int = FFERRTAG!(0xF8, b'4', b'0', b'3');
pub const AVERROR_HTTP_NOT_FOUND:    c_int = FFERRTAG!(0xF8, b'4', b'0', b'4');
pub const AVERROR_HTTP_OTHER_4XX:    c_int = FFERRTAG!(0xF8, b'4', b'X', b'X');
pub const AVERROR_HTTP_SERVER_ERROR: c_int = FFERRTAG!(0xF8, b'5', b'X', b'X');

pub const AV_ERROR_MAX_STRING_SIZE: size_t = 64;

#[inline(always)]
pub unsafe fn av_make_error_string(errbuf: *mut c_char, errbuf_size: size_t, errnum: c_int) -> *mut c_char {
	av_strerror(errnum, errbuf, errbuf_size);

	errbuf
}

extern {
	pub fn av_strerror(errnum: c_int, errbuf: *mut c_char, errbuf_size: size_t) -> c_int;
}
