use libc::{c_int, c_uint, int64_t};
use ::avfilter::filter::AVFilterContext;
use ::avutil::{AVFrame, AVPixelFormat, AVSampleFormat, AVRational};

pub const AV_BUFFERSINK_FLAG_PEEK:       c_int = 1;
pub const AV_BUFFERSINK_FLAG_NO_REQUEST: c_int = 2;

#[repr(C)]
pub struct AVBufferSinkParams {
	pub pixel_fmts: *const AVPixelFormat,
}

#[repr(C)]
pub struct AVABufferSinkParams {
	pub sample_fmts: *const AVSampleFormat,
	pub channel_layouts: *const int64_t,
	pub channel_counts: *const c_int,
	pub all_channel_counts: c_int,
	pub sample_rates: *mut c_int,
}

extern {
	pub fn av_buffersink_get_frame_flags(ctx: *mut AVFilterContext, frame: *mut AVFrame, flags: c_int) -> c_int;

	pub fn av_buffersink_params_alloc() -> *mut AVBufferSinkParams;
	pub fn av_abuffersink_params_alloc() -> *mut AVABufferSinkParams;

	pub fn av_buffersink_set_frame_size(ctx: *mut AVFilterContext, frame_size: c_uint);
	pub fn av_buffersink_get_frame_rate(ctx: *mut AVFilterContext) -> AVRational;
	pub fn av_buffersink_get_frame(ctx: *mut AVFilterContext, frame: *mut AVFrame) -> c_int;
	pub fn av_buffersink_get_samples(ctx: *mut AVFilterContext, frame: *mut AVFrame, nb_samples: c_int) -> c_int;
}
