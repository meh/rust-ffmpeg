use libc::{c_int, c_uint, uint8_t};
use super::super::avutil::{AVPixelFormat, AVRational};

pub const DV_PROFILE_BYTES: c_int = 6 * 80;

#[derive(Debug)]
#[repr(C)]
pub struct AVDVProfile {
	pub dsf: c_int,
	pub video_type: c_int,
	pub frame_size: c_int,
	pub difseg_size: c_int,
	pub n_difchan: c_int,
	pub time_base: AVRational,
	pub ltc_divisor: c_int,
	pub height: c_int,
	pub width: c_int,
	pub sar: [AVRational; 2],
	pub pix_fmt: AVPixelFormat,
	pub bpm: c_int,
	pub block_sizes: *const uint8_t,
	pub audio_stride: c_int,
	pub audio_min_samples: [c_int; 3],
	pub audio_samples_dist: [c_int; 5],
	pub audio_shuffle: [*const uint8_t; 9],
}

extern {
	pub fn av_dv_frame_profile(sys: *const AVDVProfile, frame: *const uint8_t, buf_size: c_uint) -> *const AVDVProfile;
	pub fn av_dv_codec_profile(width: c_int, height: c_int, pix_fmt: AVPixelFormat) -> *const AVDVProfile;
	pub fn av_dv_codec_profile2(width: c_int, height: c_int, pix_fmt: AVPixelFormat, frame_rate: AVRational) -> AVDVProfile;
}
