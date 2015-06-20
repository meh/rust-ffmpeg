use libc::{c_char, c_int, uint8_t, uint16_t};
use super::pixfmt::{AVPixelFormat, AVColorRange, AVColorPrimaries, AVColorTransferCharacteristic, AVColorSpace, AVChromaLocation};

#[derive(Debug)]
#[repr(C)]
pub struct AVComponentDescriptor {
	pub data: uint16_t,
}

impl AVComponentDescriptor {
	pub fn plane(&self) -> uint16_t {
		self.data >> 14
	}

	pub fn step_minus1(&self) -> uint16_t {
		self.data >> 11 & 0b111
	}

	pub fn offset_plus1(&self) -> uint16_t {
		self.data >> 8 & 0b111
	}

	pub fn shift(&self) -> uint16_t {
		self.data >> 5 & 0b111
	}

	pub fn depth_minus1(&self) -> uint16_t {
		self.data >> 1 & 0b1111
	}
}

#[derive(Debug)]
#[repr(C)]
pub struct AVPixFmtDescriptor {
	pub name: *const c_char,

	pub nb_components: uint8_t,

	pub log2_chroma_w: uint8_t,
	pub log2_chroma_h: uint8_t,

	pub flags: uint8_t,

	pub comp: [AVComponentDescriptor; 4],

	pub alias: *const c_char,
}

pub const AV_PIX_FMT_FLAG_BE:        c_int = 1 << 0;
pub const AV_PIX_FMT_FLAG_PAL:       c_int = 1 << 1;
pub const AV_PIX_FMT_FLAG_BITSTREAM: c_int = 1 << 2;
pub const AV_PIX_FMT_FLAG_HWACCEL:   c_int = 1 << 3;
pub const AV_PIX_FMT_FLAG_PLANAR:    c_int = 1 << 4;
pub const AV_PIX_FMT_FLAG_RGB:       c_int = 1 << 5;
pub const AV_PIX_FMT_FLAG_PSEUDOPAL: c_int = 1 << 6;
pub const AV_PIX_FMT_FLAG_ALPHA:     c_int = 1 << 7;

pub const PIX_FMT_BE:        c_int = AV_PIX_FMT_FLAG_BE;
pub const PIX_FMT_PAL:       c_int = AV_PIX_FMT_FLAG_PAL;
pub const PIX_FMT_BITSTREAM: c_int = AV_PIX_FMT_FLAG_BITSTREAM;
pub const PIX_FMT_HWACCEL:   c_int = AV_PIX_FMT_FLAG_HWACCEL;
pub const PIX_FMT_PLANAR:    c_int = AV_PIX_FMT_FLAG_PLANAR;
pub const PIX_FMT_RGB:       c_int = AV_PIX_FMT_FLAG_RGB;
pub const PIX_FMT_PSEUDOPAL: c_int = AV_PIX_FMT_FLAG_PSEUDOPAL;
pub const PIX_FMT_ALPHA:     c_int = AV_PIX_FMT_FLAG_ALPHA;

pub const FF_LOSS_RESOLUTION: c_int = 0x0001;
pub const FF_LOSS_DEPTH:      c_int = 0x0002;
pub const FF_LOSS_COLORSPACE: c_int = 0x0004;
pub const FF_LOSS_ALPHA:      c_int = 0x0008;
pub const FF_LOSS_COLORQUANT: c_int = 0x0010;
pub const FF_LOSS_CHROMA:     c_int = 0x0020;

extern {
	pub fn av_read_image_line(dst: *mut uint16_t, data: *const *const uint8_t, linesize: *const c_int, desc: *const AVPixFmtDescriptor, x: c_int, y: c_int, c: c_int, w: c_int, read_pal_component: c_int);
	pub fn av_write_image_line(src: *const uint16_t, data: *const *const uint8_t, linesize: *const c_int, desc: *const AVPixFmtDescriptor, x: c_int, y: c_int, c: c_int, w: c_int);

	pub fn av_get_pix_fmt(name: *const c_char) -> AVPixelFormat;
	pub fn av_get_pix_fmt_name(pix_fmt: AVPixelFormat) -> *const c_char;
	pub fn av_get_pix_fmt_string(buf: *mut c_char, buf_size: c_int, pix_fmt: AVPixelFormat) -> *mut c_char;

	pub fn av_get_bits_per_pixel(pixdesc: *const AVPixFmtDescriptor) -> c_int;
	pub fn av_get_padded_bits_per_pixel(pixdesc: *const AVPixFmtDescriptor) -> c_int;

	pub fn av_pix_fmt_desc_get(pix_fmt: AVPixelFormat) -> *const AVPixFmtDescriptor;
	pub fn av_pix_fmt_desc_next(prev: *const AVPixFmtDescriptor) -> *const AVPixFmtDescriptor;
	pub fn av_pix_fmt_desc_get_id(desc: *const AVPixFmtDescriptor) -> AVPixelFormat;

	pub fn av_pix_fmt_get_chroma_sub_sample(pix_fmt: AVPixelFormat, h_shift: *mut c_int, v_shift: *mut c_int) -> c_int;
	pub fn av_pix_fmt_count_planes(pix_fmt: AVPixelFormat) -> c_int;
	pub fn av_pix_fmt_swap_endianness(pix_fmt: AVPixelFormat) -> AVPixelFormat;

	pub fn av_get_pix_fmt_loss(dst_pix_fmt: AVPixelFormat, src_pix_fmt: AVPixelFormat, has_alpha: c_int) -> c_int;
	pub fn av_find_best_pix_fmt_of_2(dst_pix_fmt1: AVPixelFormat, dst_pix_fmt2: AVPixelFormat, src_pix_fmt: AVPixelFormat, has_alpha: c_int, loss_ptr: *mut c_int) -> AVPixelFormat;

	pub fn av_color_range_name(range: AVColorRange) -> *const c_char;
	pub fn av_color_primaries_name(primaries: AVColorPrimaries) -> *const c_char;
	pub fn av_color_transfer_name(transfer: AVColorTransferCharacteristic) -> *const c_char;
	pub fn av_color_space_name(space: AVColorSpace) -> *const c_char;
	pub fn av_chroma_location_name(location: AVChromaLocation) -> *const c_char;
}
