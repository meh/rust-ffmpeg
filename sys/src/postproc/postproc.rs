use libc::{c_void, c_char, c_int, c_uint, int8_t, uint8_t};

pub type pp_context = c_void;
pub type pp_mode    = c_void;
pub type QP_STORE_T = int8_t;

pub const PP_QUALITY_MAX: c_int = 6;

pub const PP_CPU_CAPS_MMX:     c_uint = 0x80000000;
pub const PP_CPU_CAPS_MMX2:    c_uint = 0x20000000;
pub const PP_CPU_CAPS_3DNOW:   c_uint = 0x40000000;
pub const PP_CPU_CAPS_ALTIVEC: c_uint = 0x10000000;
pub const PP_CPU_CAPS_AUTO:    c_uint = 0x00080000;

pub const PP_FORMAT:     c_int = 0x00000008;
pub const PP_FORMAT_420: c_int = 0x00000011 | PP_FORMAT;
pub const PP_FORMAT_422: c_int = 0x00000001 | PP_FORMAT;
pub const PP_FORMAT_411: c_int = 0x00000002 | PP_FORMAT;
pub const PP_FORMAT_444: c_int = 0x00000000 | PP_FORMAT;
pub const PP_FORMAT_440: c_int = 0x00000010 | PP_FORMAT;

pub const PP_PICT_TYPE_QP2: c_int = 0x00000010;

extern {
	pub fn postproc_version() -> c_uint;
	pub fn postproc_configuration() -> *const c_char;
	pub fn postproc_license() -> *const c_char;

	pub fn pp_postprocess(src: *const *const uint8_t, srcStride: *const *const c_int, dst: *mut *mut uint8_t, dstStride: *mut *mut c_int, horizontalSize: c_int, verticalSize: c_int, QP_store: *const QP_STORE_T, QP_stride: c_int, mode: *mut pp_mode, ppContext: *mut pp_context, pict_type: c_int);

	pub fn pp_get_mode_by_name_and_quality(name: *const c_char, quality: c_int) -> *mut pp_mode;
	pub fn pp_free_mode(mode: *mut pp_mode);

	pub fn pp_get_context(width: c_int, height: c_int, flags: c_int) -> *mut pp_context;
	pub fn pp_free_context(ppContext: *mut pp_context);
}
