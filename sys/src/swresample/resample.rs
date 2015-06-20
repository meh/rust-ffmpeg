use libc::{c_void, c_char, c_int, c_uint, c_double, uint8_t, int64_t};
use ::avutil::{AVClass, AVFrame, AVSampleFormat};

pub const SWR_CH_MAX: c_int = 32;

pub const SWR_FLAG_RESAMPLE: c_int = 1;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum SwrDitherType {
	SWR_DITHER_NONE = 0,
	SWR_DITHER_RECTANGULAR,
	SWR_DITHER_TRIANGULAR,
	SWR_DITHER_TRIANGULAR_HIGHPASS,

	SWR_DITHER_NS = 64,
	SWR_DITHER_NS_LIPSHITZ,
	SWR_DITHER_NS_F_WEIGHTED,
	SWR_DITHER_NS_MODIFIED_E_WEIGHTED,
	SWR_DITHER_NS_IMPROVED_E_WEIGHTED,
	SWR_DITHER_NS_SHIBATA,
	SWR_DITHER_NS_LOW_SHIBATA,
	SWR_DITHER_NS_HIGH_SHIBATA,
	SWR_DITHER_NB,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum SwrEngine {
	SWR_ENGINE_SWR,
	SWR_ENGINE_SOXR,
	SWR_ENGINE_NB,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum SwrFilterType {
	SWR_FILTER_TYPE_CUBIC,
	SWR_FILTER_TYPE_BLACKMAN_NUTTALL,
	SWR_FILTER_TYPE_KAISER,
}

pub type SwrContext = c_void;

extern {
	pub fn swresample_version() -> c_uint;
	pub fn swresample_configuration() -> *const c_char;
	pub fn swresample_license() -> *const c_char;

	pub fn swr_get_class() -> *const AVClass;

	pub fn swr_alloc() -> *mut SwrContext;
	pub fn swr_init(s: *mut SwrContext) -> c_int;
	pub fn swr_is_initialized(s: *const SwrContext) -> c_int;
	pub fn swr_alloc_set_opts(s: *mut SwrContext, out_ch_layout: int64_t, out_sample_fmt: AVSampleFormat, out_sample_rate: c_int, in_ch_layout: int64_t, in_sample_fmt: AVSampleFormat, in_sample_rate: c_int, log_offset: c_int, log_ctx: *mut c_void) -> *mut SwrContext;
	pub fn swr_free(s: *mut *mut SwrContext);
	pub fn swr_close(s: *mut SwrContext);
	pub fn swr_convert(s: *mut SwrContext, output: *mut *mut uint8_t, out_count: c_int, input: *const *const uint8_t, in_count: c_int) -> c_int;
	pub fn swr_next_pts(s: *const SwrContext, pts: int64_t) -> int64_t;
	pub fn swr_set_compensation(s: *mut SwrContext, sample_delta: c_int, compensation_distance: c_int) -> c_int;
	pub fn swr_set_channel_mapping(s: *mut SwrContext, channel_map: *const c_int) -> c_int;
	pub fn swr_set_matrix(s: *mut SwrContext, matrix: *const c_double, stride: c_int) -> c_int;
	pub fn swr_drop_output(s: *mut SwrContext, count: c_int) -> c_int;
	pub fn swr_inject_silence(s: *mut SwrContext, count: c_int) -> c_int;
	pub fn swr_get_delay(s: *const SwrContext, base: int64_t) -> int64_t;
	pub fn swr_get_out_samples(s: *const SwrContext, in_samples: c_int) -> c_int;

	pub fn swr_convert_frame(swr: *mut SwrContext, output: *mut AVFrame, input: *const AVFrame) -> c_int;
	pub fn swr_config_frame(swr: *mut SwrContext, output: *mut AVFrame, input: *const AVFrame) -> c_int;
}
