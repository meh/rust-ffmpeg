use libc::{c_char, c_int, uint8_t};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVSampleFormat {
	AV_SAMPLE_FMT_NONE = -1,
	AV_SAMPLE_FMT_U8,
	AV_SAMPLE_FMT_S16,
	AV_SAMPLE_FMT_S32,
	AV_SAMPLE_FMT_FLT,
	AV_SAMPLE_FMT_DBL,

	AV_SAMPLE_FMT_U8P,
	AV_SAMPLE_FMT_S16P,
	AV_SAMPLE_FMT_S32P,
	AV_SAMPLE_FMT_FLTP,
	AV_SAMPLE_FMT_DBLP,

	AV_SAMPLE_FMT_NB
}

extern {
	pub fn av_get_sample_fmt_name(sample_fmt: AVSampleFormat) -> *const c_char;
	pub fn av_get_sample_fmt(name: *const c_char) -> AVSampleFormat;
	pub fn av_get_alt_sample_fmt(sample_fmt: AVSampleFormat, planar: c_int) -> AVSampleFormat;
	pub fn av_get_packed_sample_fmt(sample_fmt: AVSampleFormat) -> AVSampleFormat;
	pub fn av_get_planar_sample_fmt(sample_fmt: AVSampleFormat) -> AVSampleFormat;
	pub fn av_get_sample_fmt_string(buf: *mut c_char, buf_size: c_int, sample_fmt: AVSampleFormat) -> *mut c_char;
	pub fn av_get_bytes_per_sample(sample_fmt: AVSampleFormat) -> c_int;
	pub fn av_sample_fmt_is_planar(sample_fmt: AVSampleFormat) -> c_int;
	pub fn av_samples_get_buffer_size(linesize: *mut c_int, nb_channels: c_int, nb_samples: c_int, sample_fmt: AVSampleFormat, align: c_int) -> c_int;
	pub fn av_samples_fill_arrays(audio_data: *mut *mut uint8_t, linesize: *mut c_int, buf: *const uint8_t, nb_channels: c_int, nb_samples: c_int, sample_fmt: AVSampleFormat, align: c_int) -> c_int;
	pub fn av_samples_alloc(audio_data: *mut *mut uint8_t, linesize: *mut c_int, nb_channels: c_int, nb_samples: c_int, sample_fmt: AVSampleFormat, align: c_int) -> c_int;
	pub fn av_samples_alloc_array_and_samples(audio_data: *mut *mut *mut uint8_t, linesize: *mut c_int, nb_channels: c_int, nb_samples: c_int, sample_fmt: AVSampleFormat, align: c_int) -> c_int;
	pub fn av_samples_copy(dst: *mut *mut uint8_t, src: *mut *const uint8_t, dst_offset: c_int, src_offset: c_int, nb_samples: c_int, nb_channels: c_int, sample_fmt: AVSampleFormat) -> c_int;
	pub fn av_samples_set_silence(audio_data: *mut *mut uint8_t, offset: c_int, nb_samples: c_int, nb_channels: c_int, sample_fmt: AVSampleFormat) -> c_int;
}
