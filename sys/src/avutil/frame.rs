use libc::{c_void, c_char, c_int, uint8_t, int64_t, uint64_t};
use super::dict::AVDictionary;
use super::rational::AVRational;
use super::buffer::AVBufferRef;
use super::pixfmt::{AVColorPrimaries, AVColorRange, AVColorSpace, AVColorTransferCharacteristic, AVChromaLocation};
use super::picture::AVPictureType;

#[derive(Eq, PartialEq, Debug)]
#[repr(C)]
pub enum AVFrameSideDataType {
	AV_FRAME_DATA_PANSCAN,
	AV_FRAME_DATA_A53_CC,
	AV_FRAME_DATA_STEREO3D,
	AV_FRAME_DATA_MATRIXENCODING,
	AV_FRAME_DATA_DOWNMIX_INFO,
	AV_FRAME_DATA_REPLAYGAIN,
	AV_FRAME_DATA_DISPLAYMATRIX,
	AV_FRAME_DATA_AFD,
	AV_FRAME_DATA_MOTION_VECTORS,
	AV_FRAME_DATA_SKIP_SAMPLES,
	AV_FRAME_DATA_AUDIO_SERVICE_TYPE,
}

#[derive(Debug)]
#[repr(C)]
pub struct AVFrameSideData {
	kind: AVFrameSideDataType,
	data: *mut uint8_t,
	size: c_int,
	metadata: *mut AVDictionary,
}

#[derive(Debug)]
#[repr(C)]
pub struct AVFrame {
	data: [*mut uint8_t; 8],
	linesize: [c_int; 8],
	extended_data: *mut *mut uint8_t,

	width: c_int,
	height: c_int,
	nb_samples: c_int,

	format: c_int,
	key_frame: c_int,
	pict_type: AVPictureType,
	sample_aspect_ratio: AVRational,

	pts: int64_t,
	pkt_pts: int64_t,
	pkt_dts: int64_t,

	coded_picture_number: c_int,
	display_picture_number: c_int,

	quality: c_int,

	opaque: *mut c_void,

	repeat_pict: c_int,
	interlaced_frame: c_int,
	top_field_first: c_int,

	palette_has_changed: c_int,

	reordered_opaque: int64_t,
	sample_rate: c_int,
	channel_layout: uint64_t,

	buf: [*mut AVBufferRef; 8],
	extended_buf: *mut *mut AVBufferRef,
	nb_extended_buf: c_int,

	side_data: *mut *mut AVFrameSideData,
	nb_side_data: c_int,

	flags: c_int,

	color_range: AVColorRange,
	color_primaries: AVColorPrimaries,
	color_trc: AVColorTransferCharacteristic,
	colorspace: AVColorSpace,
	chroma_location: AVChromaLocation,

	best_effort_timestamp: int64_t,
	pkt_pos: int64_t,
	pkt_duration: int64_t,

	metadata: *mut AVDictionary,
	decode_error_flags: c_int,

	channels: c_int,
	pkt_size: c_int,

	qp_table_buf: *mut AVBufferRef,
}

#[link(name = "avutil")]
extern {
	pub fn av_frame_get_best_effort_timestamp(frame: *const AVFrame) -> int64_t;
	pub fn av_frame_set_best_effort_timestamp(frame: *mut AVFrame, val: int64_t);

	pub fn av_frame_get_pkt_duration(frame: *const AVFrame) -> int64_t;
	pub fn av_frame_set_pkt_duration(frame: *mut AVFrame, val: int64_t);
	
	pub fn av_frame_get_pkt_pos(frame: *const AVFrame) -> int64_t;
	pub fn av_frame_set_pkt_pos(frame: *mut AVFrame, val: int64_t);

	pub fn av_frame_get_channel_layout(frame: *const AVFrame) -> int64_t;
	pub fn av_frame_set_channel_layout(frame: *mut AVFrame, val: int64_t);

	pub fn av_frame_get_channels(frame: *const AVFrame) -> c_int;
	pub fn av_frame_set_channels(frame: *mut AVFrame, val: c_int);

	pub fn av_frame_get_sample_rate(frame: *const AVFrame) -> c_int;
	pub fn av_frame_set_sample_rate(frame: *mut AVFrame, val: c_int);

	pub fn av_frame_get_metadata(frame: *const AVFrame) -> *mut AVDictionary;
	pub fn av_frame_set_metadata(frame: *mut AVFrame, val: *mut AVDictionary);

	pub fn av_frame_get_decode_error_flags(frame: *const AVFrame) -> c_int;
	pub fn av_frame_set_decode_error_flags(frame: *mut AVFrame, val: c_int);

	pub fn av_frame_get_pkt_size(frame: *const AVFrame) -> c_int;
	pub fn av_frame_set_pkt_size(frame: *mut AVFrame, val: c_int);

	pub fn av_frame_get_metadatap(frame: *mut AVFrame) -> *mut *mut AVDictionary;

	pub fn av_frame_get_qp_table(f: *mut AVFrame, stride: *mut c_int, kind: *mut c_int);
	pub fn av_frame_set_qp_table(f: *mut AVFrame, buf: *mut AVBufferRef, stride: c_int, kind: c_int) -> c_int;

	pub fn av_frame_get_colorspace(frame: *const AVFrame) -> AVColorSpace;
	pub fn av_frame_set_colorspace(frame: *mut AVFrame, val: AVColorSpace);

	pub fn av_frame_get_color_range(frame: *const AVFrame) -> AVColorRange;
	pub fn av_frame_set_color_range(frame: *mut AVFrame, val: AVColorRange);

	pub fn av_get_colorspace_name(val: AVColorSpace) -> *const c_char;

	pub fn av_frame_alloc() -> *mut AVFrame;
	pub fn av_frame_free(frame: *mut *mut AVFrame);
	pub fn av_frame_ref(dst: *mut AVFrame, src: *const AVFrame) -> c_int;
	pub fn av_frame_clone(src: *const AVFrame) -> *mut AVFrame;
	pub fn av_frame_unref(frame: *mut AVFrame);
	pub fn av_frame_move_ref(dst: *mut AVFrame, src: *mut AVFrame);

	pub fn av_frame_get_buffer(frame: *mut AVFrame, align: c_int) -> c_int;

	pub fn av_frame_is_writable(frame: *mut AVFrame) -> c_int;
	pub fn av_frame_make_writable(frame: *mut AVFrame) -> c_int;

	pub fn av_frame_copy(dst: *mut AVFrame, src: *const AVFrame) -> c_int;
	pub fn av_frame_copy_props(dst: *mut AVFrame, src: *const AVFrame) -> c_int;

	pub fn av_frame_get_plane_buffer(frame: *mut AVFrame, plane: c_int) -> *mut AVBufferRef;

	pub fn av_frame_new_side_data(frame: *mut AVFrame, kind: AVFrameSideDataType, size: c_int) -> *mut AVFrameSideData;
	pub fn av_frame_get_side_data(frame: *const AVFrame, kind: AVFrameSideDataType) -> *mut AVFrameSideData;
	pub fn av_frame_remove_side_data(frame: *mut AVFrame, kind: AVFrameSideDataType);
	pub fn av_frame_side_data_name(kind: AVFrameSideDataType) -> *const c_char;
}
