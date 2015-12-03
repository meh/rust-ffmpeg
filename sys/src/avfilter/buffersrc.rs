use libc::{c_int, c_uint};
use ::avfilter::filter::{AVFilterContext, AVFilterBufferRef};
use ::avutil::AVFrame;

pub const AV_BUFFERSRC_FLAG_NO_CHECK_FORMAT: c_int = 1;
pub const AV_BUFFERSRC_FLAG_NO_COPY:         c_int = 2;
pub const AV_BUFFERSRC_FLAG_PUSH:            c_int = 4;
pub const AV_BUFFERSRC_FLAG_KEEP_REF:        c_int = 8;

extern {
	#[cfg(feature = "ff_api_avfilterbuffer")]
	pub fn av_buffersrc_add_ref(buffer_src: *mut AVFilterContext, picref: *mut AVFilterBufferRef, flags: c_int) -> c_int;

	pub fn av_buffersrc_get_nb_failed_requests(buffer_src: *const AVFilterContext) -> c_uint;
	pub fn av_buffersrc_write_frame(ctx: *mut AVFilterContext, frame: *const AVFrame) -> c_int;
	pub fn av_buffersrc_add_frame(ctx: *mut AVFilterContext, frame: *mut AVFrame) -> c_int;
	pub fn av_buffersrc_add_frame_flags(buffer_src: *mut AVFilterContext, frame: *mut AVFrame, flags: c_int) -> c_int;
}
