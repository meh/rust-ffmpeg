use libc::{c_void, c_char, c_int, c_uint, c_float, c_double, uint8_t};
use ::avutil::{AVPixelFormat, AVClass};

pub const SWS_FAST_BILINEAR:        c_int = 1;
pub const SWS_BILINEAR:             c_int = 2;
pub const SWS_BICUBIC:              c_int = 4;
pub const SWS_X:                    c_int = 8;
pub const SWS_POINT:                c_int = 0x10;
pub const SWS_AREA:                 c_int = 0x20;
pub const SWS_BICUBLIN:             c_int = 0x40;
pub const SWS_GAUSS:                c_int = 0x80;
pub const SWS_SINC:                 c_int = 0x100;
pub const SWS_LANCZOS:              c_int = 0x200;
pub const SWS_SPLINE:               c_int = 0x400;
pub const SWS_SRC_V_CHR_DROP_MASK:  c_int = 0x30000;
pub const SWS_SRC_V_CHR_DROP_SHIFT: c_int = 16;
pub const SWS_PARAM_DEFAULT:        c_int = 123456;
pub const SWS_PRINT_INFO:           c_int = 0x1000;
pub const SWS_FULL_CHR_H_INT:       c_int = 0x2000;
pub const SWS_FULL_CHR_H_INP:       c_int = 0x4000;
pub const SWS_DIRECT_BGR:           c_int = 0x8000;
pub const SWS_ACCURATE_RND:         c_int = 0x40000;
pub const SWS_BITEXACT:             c_int = 0x80000;
pub const SWS_ERROR_DIFFUSION:      c_int = 0x800000;

pub const SWS_CPU_CAPS_MMX:     c_uint = 0x80000000;
pub const SWS_CPU_CAPS_MMXEXT:  c_uint = 0x20000000;
pub const SWS_CPU_CAPS_MMX2:    c_uint = 0x20000000;
pub const SWS_CPU_CAPS_3DNOW:   c_uint = 0x40000000;
pub const SWS_CPU_CAPS_ALTIVEC: c_uint = 0x10000000;
pub const SWS_CPU_CAPS_BFIN:    c_uint = 0x01000000;
pub const SWS_CPU_CAPS_SSE2:    c_uint = 0x02000000;

pub const SWS_MAX_REDUCE_CUTOFF: c_float = 0.002;

pub const SWS_CS_ITU709:    c_int = 1;
pub const SWS_CS_FCC:       c_int = 4;
pub const SWS_CS_ITU601:    c_int = 5;
pub const SWS_CS_ITU624:    c_int = 5;
pub const SWS_CS_SMPTE170M: c_int = 5;
pub const SWS_CS_SMPTE240M: c_int = 7;
pub const SWS_CS_DEFAULT:   c_int = 5;

#[derive(Debug)]
#[repr(C)]
pub struct SwsVector {
	pub coeff: *mut c_double,
	pub length: c_int,
}

#[derive(Debug)]
#[repr(C)]
pub struct SwsFilter {
	pub lumH: *mut SwsVector,
	pub lumV: *mut SwsVector,
	pub chrH: *mut SwsVector,
	pub chrV: *mut SwsVector,
}

pub type SwsContext = c_void;

extern {
	pub fn swscale_version() -> c_uint;
	pub fn swscale_configuration() -> *const c_char;
	pub fn swscale_license() -> *const c_char;

	pub fn sws_getCoefficients(colorspace: c_int) -> *const c_int;

	pub fn sws_isSupportedInput(pix_fmt: AVPixelFormat) -> c_int;
	pub fn sws_isSupportedOutput(pix_fmt: AVPixelFormat) -> c_int;
	pub fn sws_isSupportedEndiannessConversion(pix_fmt: AVPixelFormat) -> c_int;

	pub fn sws_alloc_context() -> *mut SwsContext;
	pub fn sws_init_context(sws_context: *mut SwsContext, srcFilter: *mut SwsFilter, dstFilter: *mut SwsFilter) -> c_int;
	pub fn sws_freeContext(sws_context: *mut SwsContext);
	pub fn sws_getContext(srcW: c_int, srcH: c_int, srcFormat: AVPixelFormat, dstW: c_int, dstH: c_int, dstFormat: AVPixelFormat, flags: c_int, srcFilter: *mut SwsFilter, dstFilter: *mut SwsFilter, param: *const c_double) -> *mut SwsContext;

	pub fn sws_scale(c: *mut SwsContext, srcSlice: *const *const uint8_t, srcStride: *const c_int, srcSliceY: c_int, srcSliceH: c_int, dst: *mut *mut uint8_t, dstStride: *mut c_int) -> c_int;

	pub fn sws_setColorspaceDetails(c: *mut SwsContext, inv_table: *const *const c_int, srcRange: c_int, table: *const *const c_int, dstRange: c_int, brightness: c_int, contrast: c_int, saturation: c_int) -> c_int;
	pub fn sws_getColorspaceDetails(c: *mut SwsContext, inv_table: *mut *mut c_int, srcRange: *mut c_int, table: *mut *mut c_int, dstRange: *mut c_int, brightness: *mut c_int, contrast: *mut c_int, saturation: *mut c_int) -> c_int;

	pub fn sws_allocVec(length: c_int) -> *mut SwsVector;
	pub fn sws_getGaussianVec(variance: c_double, quality: c_double) -> *mut SwsVector;
	pub fn sws_getConstVec(c: c_double, length: c_int) -> *mut SwsVector;
	pub fn sws_getIdentityVec() -> *mut SwsVector;

	pub fn sws_scaleVec(a: *mut SwsVector, scalar: c_double);
	pub fn sws_normalizeVec(a: *mut SwsVector, height: c_double);
	pub fn sws_convVec(a: *mut SwsVector, b: *const SwsVector);
	pub fn sws_addVec(a: *mut SwsVector, b: *const SwsVector);
	pub fn sws_subVec(a: *mut SwsVector, b: *const SwsVector);
	pub fn sws_shiftVec(a: *mut SwsVector, shift: c_int);

	pub fn sws_cloneVec(a: *const SwsVector) -> *mut SwsVector;
	pub fn sws_printVec2(a: *const SwsVector, log_ctx: *mut AVClass, log_level: c_int);
	pub fn sws_freeVec(a: *mut SwsVector);

	pub fn sws_getDefaultFilter(lumaGBlur: c_float, chromaGBlur: c_float, lumaSharpen: c_float, chromaSharpen: c_float, chromaHShift: c_float, chromaVShift: c_float, verbose: c_int) -> *mut SwsFilter;
	pub fn sws_freeFilter(filter: *mut SwsFilter);

	pub fn sws_getCachedContext(context: *mut SwsContext, srcW: c_int, srcH: c_int, srcFormat: AVPixelFormat, dstW: c_int, dstH: c_int, dstFormat: AVPixelFormat, flags: c_int, srcFilter: *mut SwsFilter, dstFilter: *mut SwsFilter, param: *const c_double) -> *mut SwsContext;
	pub fn sws_convertPalette8ToPacked32(src: *const uint8_t, dst: *mut uint8_t, num_pixels: c_int, palette: *const uint8_t);
	pub fn sws_convertPalette8ToPacked24(src: *const uint8_t, dst: *mut uint8_t, num_pixels: c_int, palette: *const uint8_t);

	pub fn sws_get_class() -> *const AVClass;
}
