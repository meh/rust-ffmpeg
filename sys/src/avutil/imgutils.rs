use libc::{c_void, c_int, c_uint, uint8_t};
use super::pixdesc::AVPixFmtDescriptor;
use super::pixfmt::AVPixelFormat;
use super::rational::AVRational;

extern {
	pub fn av_image_fill_max_pixsteps(max_pixsteps: *mut c_int, max_pixstep_comps: *mut c_int, pixdesc: *const AVPixFmtDescriptor);
	pub fn av_image_get_linesize(pix_fmt: AVPixelFormat, width: c_int, plane: c_int) -> c_int;
	pub fn av_image_fill_linesizes(linesizes: *mut c_int, pix_fmt: AVPixelFormat, width: c_int) -> c_int;
	pub fn av_image_fill_pointers(data: *mut *mut uint8_t, pix_fmt: AVPixelFormat, height: c_int, ptr: *mut uint8_t, linesizes: *const c_int) -> c_int;

	pub fn av_image_alloc(pointers: *mut *mut uint8_t, linesizes: *mut c_int, w: c_int, h: c_int, pix_fmt: AVPixelFormat, align: c_int) -> c_int;

	pub fn av_image_copy_plane(dst: *mut uint8_t, dst_linesize: c_int, src: *const uint8_t, src_linesize: c_int, bytewidth: c_int, height: c_int);
	pub fn av_image_copy(dst_data: *mut *mut uint8_t, dst_linesizes: *mut c_int, src_data: *const *const uint8_t, src_linesizes: *const c_int, pix_fmt: AVPixelFormat, width: c_int, height: c_int);

	pub fn av_image_fill_arrays(dst_data: *mut *mut uint8_t, dst_linesize: *mut c_int, src: *const uint8_t, pix_fmt: AVPixelFormat, width: c_int, height: c_int, align: c_int) -> c_int;
	pub fn av_image_get_buffer_size(pix_fmt: AVPixelFormat, width: c_int, height: c_int, align: c_int) -> c_int;
	pub fn av_image_copy_to_buffer(dst: *mut uint8_t, dst_size: c_int, src_data: *const *const uint8_t, src_linesize: *const c_int, pix_fmt: AVPixelFormat, width: c_int, height: c_int, align: c_int) -> c_int;
	pub fn av_image_check_size(w: c_uint, h: c_uint, log_offset: c_int, log_ctx: *mut c_void) -> c_int;
	pub fn av_image_check_sar(w: c_uint, h: c_uint, sar: AVRational) -> c_int;
}
