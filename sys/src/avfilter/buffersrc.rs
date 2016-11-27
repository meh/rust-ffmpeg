use libc::{c_int, c_uint, int64_t};
use ::avfilter::filter::{AVFilterContext};
use ::avutil::{AVFrame, AVRational, AVBufferRef};

pub const AV_BUFFERSRC_FLAG_NO_CHECK_FORMAT: c_int = 1;
pub const AV_BUFFERSRC_FLAG_PUSH:            c_int = 4;
pub const AV_BUFFERSRC_FLAG_KEEP_REF:        c_int = 8;

#[repr(C)]
pub struct AVBufferSrcParameters {
	pub format: c_int,
	pub time_base: AVRational,
	pub width: c_int,
	pub height: c_int,
	pub sample_aspect_ratio: AVRational,
	pub frame_rate: AVRational,
	pub hw_frames_ctx: *mut AVBufferRef,
	pub sample_rate: c_int,
	pub channel_layout: int64_t,
}

extern {
	pub fn av_buffersrc_get_nb_failed_requests(buffer_src: *const AVFilterContext) -> c_uint;

	pub fn av_buffersrc_parameters_alloc() -> *mut AVBufferSrcParameters;
	pub fn av_buffersrc_parameters_set(ctx: *mut AVFilterContext, param: *mut AVBufferSrcParameters) -> c_int;

	pub fn av_buffersrc_write_frame(ctx: *mut AVFilterContext, frame: *const AVFrame) -> c_int;
	pub fn av_buffersrc_add_frame(ctx: *mut AVFilterContext, frame: *mut AVFrame) -> c_int;
	pub fn av_buffersrc_add_frame_flags(buffer_src: *mut AVFilterContext, frame: *mut AVFrame, flags: c_int) -> c_int;
}
