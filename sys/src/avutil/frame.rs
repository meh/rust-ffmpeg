use libc::{c_void, c_char, c_int, int8_t, uint8_t};
use libc::{int64_t, uint64_t};
use super::dict::AVDictionary;
use super::rational::AVRational;
use super::buffer::AVBufferRef;
use super::pixfmt::{AVColorPrimaries, AVColorRange, AVColorSpace, AVColorTransferCharacteristic, AVChromaLocation};
use super::util::AVPictureType;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
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
	AV_FRAME_DATA_MASTERING_DISPLAY_METADATA,
	AV_FRAME_DATA_GOP_TIMECODE,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVActiveFormatDescription {
	AV_AFD_SAME         = 8,
	AV_AFD_4_3          = 9,
	AV_AFD_16_9         = 10,
	AV_AFD_14_9         = 11,
	AV_AFD_4_3_SP_14_9  = 13,
	AV_AFD_16_9_SP_14_9 = 14,
	AV_AFD_SP_4_3       = 15,
}

#[derive(Debug)]
#[repr(C)]
pub struct AVFrameSideData {
	pub kind: AVFrameSideDataType,
	pub data: *mut uint8_t,
	pub size: c_int,
	pub metadata: *mut AVDictionary,
	pub buf: *mut AVBufferRef,
}

pub const AV_NUM_DATA_POINTERS: usize = 8;

pub const AV_FRAME_FLAG_CORRUPT: c_int = 1 << 0;
pub const AV_FRAME_FLAG_DISCARD: c_int = 1 << 2;

pub const FF_DECODE_ERROR_INVALID_BITSTREAM: c_int = 1;
pub const FF_DECODE_ERROR_MISSING_REFERENCE: c_int = 2;

#[derive(Debug)]
#[repr(C)]
pub struct AVFrame {
	pub data: [*mut uint8_t; AV_NUM_DATA_POINTERS],
	pub linesize: [c_int; AV_NUM_DATA_POINTERS],
	pub extended_data: *mut *mut uint8_t,

	pub width: c_int,
	pub height: c_int,
	pub nb_samples: c_int,

	pub format: c_int,
	pub key_frame: c_int,
	pub pict_type: AVPictureType,

	pub sample_aspect_ratio: AVRational,
	pub pts: int64_t,
	#[cfg(feature = "ff_api_pkt_pts")]
	pub pkt_pts: int64_t,
	pub pkt_dts: int64_t,

	pub coded_picture_number: c_int,
	pub display_picture_number: c_int,

	pub quality: c_int,

	pub opaque: *mut c_void,
	#[cfg(feature = "ff_api_error_frame")]
	pub error: [uint64_t; AV_NUM_DATA_POINTERS],

	pub repeat_pict: c_int,
	pub interlaced_frame: c_int,
	pub top_field_first: c_int,

	pub palette_has_changed: c_int,

	pub reordered_opaque: int64_t,

	pub sample_rate: c_int,
	pub channel_layout: uint64_t,

	pub buf: [*mut AVBufferRef; AV_NUM_DATA_POINTERS],
	pub extended_buf: *mut *mut AVBufferRef,
	pub nb_extended_buf: c_int,

	pub side_data: *mut *mut AVFrameSideData,
	pub nb_side_data: c_int,

	pub flags: c_int,

	pub color_range: AVColorRange,
	pub color_primaries: AVColorPrimaries,
	pub color_trc: AVColorTransferCharacteristic,
	pub colorspace: AVColorSpace,
	pub chroma_location: AVChromaLocation,

	pub best_effort_timestamp: int64_t,
	pub pkt_pos: int64_t,
	pub pkt_duration: int64_t,

	pub metadata: *mut AVDictionary,
	pub decode_error_flags: c_int,

	pub channels: c_int,
	pub pkt_size: c_int,

	#[cfg(feature = "ff_api_frame_qp")]
	pub qscale_table: *mut int8_t,
	#[cfg(feature = "ff_api_frame_qp")]
	pub qstride: c_int,
	#[cfg(feature = "ff_api_frame_qp")]
	pub qscale_type: c_int,
	#[cfg(feature = "ff_api_frame_qp")]
	pub qp_table_buf: *mut AVBufferRef,

	pub hw_frames_ctx: *mut AVBufferRef,
}

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
