use libc::{c_void, c_char, c_int, c_uint, c_double, uint8_t, uint64_t};
use ::avutil::{AVClass, AVMatrixEncoding, AVFrame};

pub const AVRESAMPLE_MAX_CHANNELS: c_int = 32;

pub type AVAudioResampleContext = c_void;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVMixCoeffType {
	AV_MIX_COEFF_TYPE_Q8,
	AV_MIX_COEFF_TYPE_Q15,
	AV_MIX_COEFF_TYPE_FLT,
	AV_MIX_COEFF_TYPE_NB,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVResampleFilterType {
	AV_RESAMPLE_FILTER_TYPE_CUBIC,
	AV_RESAMPLE_FILTER_TYPE_BLACKMAN_NUTTALL,
	AV_RESAMPLE_FILTER_TYPE_KAISER,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVResampleDitherMethod {
	AV_RESAMPLE_DITHER_NONE,
	AV_RESAMPLE_DITHER_RECTANGULAR,
	AV_RESAMPLE_DITHER_TRIANGULAR,
	AV_RESAMPLE_DITHER_TRIANGULAR_HP,
	AV_RESAMPLE_DITHER_TRIANGULAR_NS,
	AV_RESAMPLE_DITHER_NB,
}

extern {
	pub fn avresample_version() -> c_uint;
	pub fn avresample_configuration() -> *const c_char;
	pub fn avresample_license() -> *const c_char;

	pub fn avresample_get_class() -> *const AVClass;

	pub fn avresample_alloc_context() -> *mut AVAudioResampleContext;
	pub fn avresample_open(avr: *mut AVAudioResampleContext) -> c_int;
	pub fn avresample_is_open(avr: *mut AVAudioResampleContext) -> c_int;
	pub fn avresample_close(avr: *mut AVAudioResampleContext);
	pub fn avresample_free(avr: *mut *mut AVAudioResampleContext);

	pub fn avresample_build_matrix(in_layout: uint64_t, out_layout: uint64_t, center_mix_level: c_double, surround_mix_level: c_double, lfe_mix_level: c_double, normalize: c_int, matrix: *mut c_double, stride: c_int, matrix_encoding: AVMatrixEncoding) -> c_int;
	pub fn avresample_get_matrix(avr: *mut AVAudioResampleContext, matrix: *mut c_double, stride: c_int) -> c_int;
	pub fn avresample_set_matrix(avr: *mut AVAudioResampleContext, matrix: *const c_double, stride: c_int) -> c_int;
	pub fn avresample_set_channel_mapping(avr: *mut AVAudioResampleContext, channel_map: *const c_int) -> c_int;
	pub fn avresample_set_compensation(avr: *mut AVAudioResampleContext, sample_delta: c_int, compensation_distance: c_int) -> c_int;
	pub fn avresample_get_out_samples(avr: *mut AVAudioResampleContext, in_nb_samples: c_int) -> c_int;
	pub fn avresample_convert(avr: *mut AVAudioResampleContext, output: *mut *mut uint8_t, out_plane_size: c_int, out_samples: c_int, input: *mut *mut uint8_t, in_plane_size: c_int, in_samples: c_int) -> c_int;
	pub fn avresample_get_delay(avr: *mut AVAudioResampleContext) -> c_int;
	pub fn avresample_available(avr: *mut AVAudioResampleContext) -> c_int;
	pub fn avresample_read(avr: *mut AVAudioResampleContext, output: *mut *mut uint8_t, nb_samples: c_int) -> c_int;
	pub fn avresample_convert_frame(avr: *mut AVAudioResampleContext, output: *mut AVFrame, input: *const AVFrame) -> c_int;
	pub fn avresample_config(avr: *mut AVAudioResampleContext, output: *mut AVFrame, input: *const AVFrame) -> c_int;
}
