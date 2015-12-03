use libc::{c_void, c_char, c_int, c_uint, c_double, int8_t, uint8_t, int64_t, uint64_t};
use ::avutil::{AVClass, AVRational, AVPictureType, AVMediaType, AVDictionary, AVFrame};

#[repr(C)]
pub struct AVFilterBuffer {
	pub data: [*mut uint8_t; 8],
	pub extended_data: *mut *mut uint8_t,
	pub linesize: [c_int; 8],
	pub private: *mut c_void,
	pub free: extern fn(*mut AVFilterBuffer),
	pub format: c_int,
	pub w: c_int,
	pub h: c_int,
	pub refcount: c_uint,
}

pub const AV_PERM_READ:          c_int = 0x01;
pub const AV_PERM_WRITE:         c_int = 0x02;
pub const AV_PERM_PRESERVE:      c_int = 0x04;
pub const AV_PERM_REUSE:         c_int = 0x08;
pub const AV_PERM_REUSE2:        c_int = 0x10;
pub const AV_PERM_NEG_LINESIZES: c_int = 0x20;
pub const AV_PERM_ALIGN:         c_int = 0x40;

pub const AVFILTER_ALIGN: c_int = 16;

#[repr(C)]
pub struct AVFilterBufferRefAudioProps {
	pub channel_layout: uint64_t,
	pub nb_samples: c_int,
	pub sample_rate: c_int,
	pub channels: c_int,
}

#[repr(C)]
pub struct AVFilterBufferRefVideoProps {
	pub w: c_int,
	pub h: c_int,
	pub sample_aspect_ratio: AVRational,
	pub interlaced: c_int,
	pub top_field_first: c_int,
	pub pict_type: AVPictureType,
	pub key_frame: c_int,
	pub qp_table_linesize: c_int,
	pub qp_table_size: c_int,
	pub qp_table: *mut int8_t,
}

#[repr(C)]
pub struct AVFilterBufferRef {
	pub buf: *mut AVFilterBuffer,
	pub data: [*mut uint8_t; 8],
	pub extended_data: *mut *mut uint8_t,
	pub linesize: [c_int; 8],
	pub video: *mut AVFilterBufferRefVideoProps,
	pub audio: *mut AVFilterBufferRefAudioProps,
	pub pts: int64_t,
	pub pos: int64_t,
	pub format: c_int,
	pub perms: c_int,
	pub kind: AVMediaType,
	pub metadata: *mut AVDictionary,
}

#[repr(C)]
pub struct AVFilterPad {
	pub name: *const c_char,
	pub kind: AVMediaType,
	pub min_perms: c_int,
	pub rej_perms: c_int,
	pub start_frame: Option<extern fn(*mut AVFilterLink, *mut AVFilterBufferRef) -> c_int>,
	pub get_video_buffer: Option<extern fn(*mut AVFilterLink, c_int, c_int) -> *mut AVFrame>,
	pub get_audio_buffer: Option<extern fn(*mut AVFilterLink, c_int) -> *mut AVFrame>,
	pub end_frame: Option<extern fn(*mut AVFilterLink) -> c_int>,
	pub draw_slice: Option<extern fn(*mut AVFilterLink, c_int, c_int, c_int) -> c_int>,
	pub filter_frame: extern fn(*mut AVFilterLink, *mut AVFrame) -> c_int,
	pub poll_frame: extern fn(*mut AVFilterLink) -> c_int,
	pub request_frame: extern fn(*mut AVFilterLink) -> c_int,
	pub config_props: extern fn(*mut AVFilterLink) -> c_int,
	pub needs_fifo: c_int,
	pub needs_writable: c_int,
}

pub const AVFILTER_FLAG_DYNAMIC_INPUTS:            c_int = 1 << 0;
pub const AVFILTER_FLAG_DYNAMIC_OUTPUTS:           c_int = 1 << 1;
pub const AVFILTER_FLAG_SLICE_THREADS:             c_int = 1 << 2;
pub const AVFILTER_FLAG_SUPPORT_TIMELINE_GENERIC:  c_int = 1 << 16;
pub const AVFILTER_FLAG_SUPPORT_TIMELINE_INTERNAL: c_int = 1 << 17;
pub const AVFILTER_FLAG_SUPPORT_TIMELINE:          c_int = AVFILTER_FLAG_SUPPORT_TIMELINE_GENERIC | AVFILTER_FLAG_SUPPORT_TIMELINE_INTERNAL;

#[repr(C)]
pub struct AVFilter {
	pub name: *const c_char,
	pub description: *const c_char,
	pub inputs: *const AVFilterPad,
	pub outputs: *const AVFilterPad,
	pub priv_class: *const AVClass,
	pub flags: c_int,

	pub init: extern fn(*mut AVFilterContext) -> c_int,
	pub init_dict: extern fn(*mut AVFilterContext, *mut *mut AVDictionary) -> c_int,
	pub uninit: extern fn(*mut AVFilterContext),
	pub query_formats: extern fn(*mut AVFilterContext) -> c_int,
	pub priv_size: c_int,
	pub next: *mut AVFilter,
	pub process_command: extern fn(*mut AVFilterContext, cmd: *const c_char, arg: *const c_char, *mut c_char, c_int, c_int) -> c_int,
	pub init_opaque: extern fn(*mut AVFilterContext, *mut c_void) -> c_int,
}

pub const AVFILTER_THREAD_SLICE: c_int = 1 << 0;

pub type AVFilterInternal = c_void;

#[repr(C)]
pub struct AVFilterContext {
	pub av_class: *const AVClass,
	pub filter: *const AVFilter,
	pub name: *mut c_char,
	pub input_pads: *mut AVFilterPad,
	pub inputs: *mut *mut AVFilterLink,
	pub input_count: c_uint,
	pub nb_inputs: c_uint,
	pub output_pads: *mut AVFilterPad,
	pub outputs: *mut *mut AVFilterLink,
	pub output_count: c_uint,
	pub nb_outputs: c_uint,
	pub private: *mut c_void,
	pub graph: *mut AVFilterGraph,
	pub thread_type: c_int,
	pub internal: *mut AVFilterInternal,
	pub command_queue: *mut AVFilterCommand,
	pub enable_str: *mut c_char,
	pub enable: *mut c_void,
	pub var_values: *mut c_double,
	pub is_disabled: c_int,
}

pub const AVLINK_UNINIT:    c_int = 0;
pub const AVLINK_STARTINIT: c_int = 1;
pub const AVLINK_INIT:      c_int = 2;

#[repr(C)]
pub struct AVFilterLink {
	pub src: *mut AVFilterContext,
	pub srcpad: *mut AVFilterPad,
	pub dst: *mut AVFilterContext,
	pub dstpad: *mut AVFilterPad,
	pub kind: AVMediaType,
	pub w: c_int,
	pub h: c_int,
	pub sample_aspect_ratio: AVRational,
	pub channel_layout: uint64_t,
	pub sample_rate: c_int,
	pub format: c_int,
	pub time_base: AVRational,

	pub in_formats: *mut AVFilterFormats,
	pub out_formats: *mut AVFilterFormats,
	pub in_channel_layouts: *mut AVFilterChannelLayouts,
	pub out_channel_layouts: *mut AVFilterChannelLayouts,
	pub request_samples: c_int,
	#[cfg(feature = "ff_api_avfilterbuffer")]
	pub pool: *mut AVFilterPool,
	pub graph: *mut AVFilterGraph,
	pub current_pts: int64_t,
	pub age_index: c_int,
	pub frame_rate: AVRational,
	pub partial_buf: *mut AVFrame,
	pub partial_buf_size: c_int,
	pub min_samples: c_int,
	pub max_samples: c_int,
	#[cfg(feature = "ff_api_avfilterbuffer")]
	pub cur_buf_copy: *mut AVFilterBufferRef,
	pub closed: c_int,
	pub channels: c_int,
	pub frame_requested: c_uint,
	pub flags: c_uint,
	pub frame_count: int64_t,
}

pub const AVFILTER_CMD_FLAG_ONE:  c_int = 1;
pub const AVFILTER_CMD_FLAG_FAST: c_int = 2;

pub type AVFilterGraphInternal = c_void;

pub type avfilter_action_func = extern fn(*mut AVFilterContext, *mut c_void, c_int, c_int) -> c_int;
pub type avfilter_execute_func = extern fn(*mut AVFilterContext, avfilter_action_func, *mut c_void, *mut c_int, c_int) -> c_int;

#[repr(C)]
pub struct AVFilterGraph {
	pub av_class: *const AVClass,
	pub filter_count_unused: c_uint,
	pub filters: *mut *mut AVFilterContext,
	pub scale_sws_opts: *mut c_char,
	pub resample_lavr_opts: *mut c_char,
	pub nb_filters: c_uint,
	pub thread_type: c_int,
	pub nb_threads: c_int,
	pub internal: *mut AVFilterGraphInternal,
	pub opaque: *mut c_void,
	pub execute: avfilter_execute_func,
	pub aresample_swr_opts: *mut c_char,
	pub sink_links: *mut *mut AVFilterLink,
	pub sink_links_count: c_int,
	pub disable_auto_convert: c_uint,
}

pub const AVFILTER_AUTO_CONVERT_ALL:  c_int = 0;
pub const AVFILTER_AUTO_CONVERT_NONE: c_int = -1;

pub type AVFilterCommand        = c_void;
pub type AVFilterChannelLayouts = c_void;
pub type AVFilterPool           = c_void;
pub type AVFilterFormats        = c_void;

#[repr(C)]
pub struct AVFilterInOut {
	pub name: *mut c_char,
	pub filter_ctx: *mut AVFilterContext,
	pub pad_idx: c_int,
	pub next: *mut AVFilterInOut,
}

extern {
	pub fn avfilter_version() -> c_uint;
	pub fn avfilter_configuration() -> *const c_char;
	pub fn avfilter_license() -> *const c_char;

	pub fn avfilter_pad_count(pads: *const AVFilterPad) -> c_int;
	pub fn avfilter_pad_get_name(pads: *const AVFilterPad, pad_idx: c_int) -> *const c_char;
	pub fn avfilter_pad_get_type(pads: *const AVFilterPad, pad_idx: c_int) -> AVMediaType;

	pub fn avfilter_link(src: *mut AVFilterContext, srcpad: c_uint, dst: *mut AVFilterContext, dstpad: c_uint) -> c_int;
	pub fn avfilter_link_free(link: *mut *mut AVFilterLink);
	pub fn avfilter_link_get_channels(link: *mut AVFilterLink) -> c_int;
	pub fn avfilter_link_set_closed(link: *mut AVFilterLink, closed: c_int);
	pub fn avfilter_config_links(filter: *mut AVFilterContext) -> c_int;

	pub fn avfilter_process_command(filter: *mut AVFilterContext, cmd: *const c_char, arg: *const c_char, res: *const c_char, res_len: c_int, flags: c_int) -> c_int;

	pub fn avfilter_register_all();
	pub fn avfilter_register(filter: *const AVFilter) -> c_int;

	pub fn avfilter_get_by_name(name: *const c_char) -> *mut AVFilter;
	pub fn avfilter_next(prev: *const AVFilter) -> *const AVFilter;

	pub fn avfilter_init_str(ctx: *mut AVFilterContext, args: *const c_char) -> c_int;
	pub fn avfilter_init_dict(ctx: *mut AVFilterContext, options: *mut *mut AVDictionary) -> c_int;
	pub fn avfilter_free(filter: *mut AVFilterContext);
	pub fn avfilter_insert_filter(link: *mut AVFilterLink, filt: *mut AVFilterContext, filt_srcpad_idx: c_uint, filt_dstpad_idx: c_uint) -> c_int;

	pub fn avfilter_get_class() -> *const AVClass;

	pub fn avfilter_graph_alloc() -> *mut AVFilterGraph;
	pub fn avfilter_graph_alloc_filter(graph: *mut AVFilterGraph, filter: *const AVFilter, name: *const c_char) -> *mut AVFilterContext;
	pub fn avfilter_graph_get_filter(graph: *mut AVFilterGraph, name: *const c_char) -> *mut AVFilterContext;
	pub fn avfilter_graph_create_filter(filt_ctx: *mut *mut AVFilterContext, filt: *const AVFilter, name: *const c_char, args: *const c_char, opaque: *mut c_void, graph_ctx: *mut AVFilterGraph) -> c_int;
	pub fn avfilter_graph_set_auto_convert(graph: *mut AVFilterGraph, flags: c_uint);
	pub fn avfilter_graph_config(graphctx: *mut AVFilterGraph, log_ctx: *mut c_void) -> c_int;
	pub fn avfilter_graph_free(graph: *mut *mut AVFilterGraph);

	pub fn avfilter_inout_alloc() -> *mut AVFilterInOut;
	pub fn avfilter_inout_free(inout: *mut *mut AVFilterInOut);

	pub fn avfilter_graph_parse(graph: *mut AVFilterGraph, filters: *const c_char, inputs: *mut AVFilterInOut, outputs: *mut AVFilterInOut, log_ctx: *mut c_void) -> c_int;
	pub fn avfilter_graph_parse_ptr(graph: *mut AVFilterGraph, filters: *const c_char, inputs: *mut *mut AVFilterInOut, outputs: *mut *mut AVFilterInOut, log_ctx: *mut c_void) -> c_int;
	pub fn avfilter_graph_parse2(graph: *mut AVFilterGraph, filters: *const c_char, inputs: *mut *mut AVFilterInOut, outputs: *mut *mut AVFilterInOut) -> c_int;
	pub fn avfilter_graph_send_command(graph: *mut AVFilterGraph, target: *const c_char, cmd: *const c_char, arg: *const c_char, res: *mut c_char, res_len: c_int, flags: c_int) -> c_int;
	pub fn avfilter_graph_queue_command(graph: *mut AVFilterGraph, target: *const c_char, cmd: *const c_char, arg: *const c_char, flags: c_int, ts: c_double) -> c_int;
	pub fn avfilter_graph_dump(graph: *const AVFilterGraph, options: *const c_char) -> *mut c_char;
	pub fn avfilter_graph_request_oldest(graph: *mut AVFilterGraph) -> c_int;
}
