use libc::{c_void, c_char, c_uchar, c_int, c_uint, c_double, uint8_t, int64_t, size_t, FILE};
use super::io::{AVIOContext, AVIOInterruptCB};
use super::super::avutil::{AVClass, AVRational, AVDictionary, AVFrame, AVMediaType};
use super::super::avcodec::{AVCodec, AVCodecID, AVCodecContext, AVPacket, AVPacketSideData, AVCodecParserContext, AVDiscard};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub struct AVFrac {
	pub val: int64_t,
	pub num: int64_t,
	pub den: int64_t,
}

pub type AVCodecTag = c_void;

#[derive(Debug)]
#[repr(C)]
pub struct AVProbeData {
	pub filename: *const c_char,
	pub buf: *mut c_uchar,
	pub buf_size: c_int,
	pub mime_type: *const c_char,
}

pub const AVPROBE_SCORE_EXTENSION:    c_int = 50;
pub const AVPROBE_SCORE_MIME:         c_int = 75;
pub const AVPROBE_SCORE_MAX:          c_int = 100;
pub const AVPROBE_SCORE_RETRY:        c_int = AVPROBE_SCORE_MAX / 4;
pub const AVPROBE_SCORE_STREAM_RETRY: c_int = AVPROBE_SCORE_MAX / 4 - 1;

pub const AVPROBE_PADDING_SIZE: c_int = 32;

pub const AVFMT_NOFILE:        c_int = 0x0001;
pub const AVFMT_NEEDNUMBER:    c_int = 0x0002;
pub const AVFMT_SHOW_IDS:      c_int = 0x0008;
pub const AVFMT_RAWPICTURE:    c_int = 0x0020;
pub const AVFMT_GLOBALHEADER:  c_int = 0x0040;
pub const AVFMT_NOTIMESTAMPS:  c_int = 0x0080;
pub const AVFMT_GENERIC_INDEX: c_int = 0x0100;
pub const AVFMT_TS_DISCONT:    c_int = 0x0200;
pub const AVFMT_VARIABLE_FPS:  c_int = 0x0400;
pub const AVFMT_NODIMENSIONS:  c_int = 0x0800;
pub const AVFMT_NOSTREAMS:     c_int = 0x1000;
pub const AVFMT_NOBINSEARCH:   c_int = 0x2000;
pub const AVFMT_NOGENSEARCH:   c_int = 0x4000;
pub const AVFMT_NO_BYTE_SEEK:  c_int = 0x8000;
pub const AVFMT_ALLOW_FLUSH:   c_int = 0x10000;
pub const AVFMT_TS_NONSTRICT:  c_int = 0x20000;
pub const AVFMT_TS_NEGATIVE:   c_int = 0x40000;
pub const AVFMT_SEEK_TO_PTS:   c_int = 0x4000000;

#[repr(C)]
pub struct AVOutputFormat {
	pub name:      *const c_char,
	pub long_name: *const c_char,

	pub mime_type:  *const c_char,
	pub extensions: *const c_char,

	pub audio_codec:    AVCodecID,
	pub video_codec:    AVCodecID,
	pub subtitle_codec: AVCodecID,

	pub flags: c_int,
	pub codec_tag:  *const *const AVCodecTag,
	pub priv_class: *const AVClass,
	pub next: *mut AVOutputFormat,
	pub priv_data_size: c_int,

	pub write_header:  extern fn(ctx: *mut AVFormatContext) -> c_int,
	pub write_packet:  extern fn(ctx: *mut AVFormatContext, pkt: *mut AVPacket) -> c_int,
	pub write_trailer: extern fn(ctx: *mut AVFormatContext) -> c_int,

	pub interleave_packet: extern fn(ctx: *mut AVFormatContext, out: *mut AVPacket, inp: *mut AVPacket, flush: c_int) -> c_int,
	pub query_codec: extern fn(id: AVCodecID, std_compliance: c_int) -> c_int,
	pub get_output_timestamp: extern fn(ctx: *mut AVFormatContext, stream: c_int, dts: *mut int64_t, wall: *mut int64_t),
	pub control_message: extern fn(ctx: *mut AVFormatContext, kind: c_int, data: *mut c_void, data_size: size_t) -> c_int,
	pub write_uncoded_frame: extern fn(ctx: *mut AVFormatContext, stream_index: c_int, frame: *mut *mut AVFrame, flags: c_uint) -> c_int,

	pub get_device_list: extern fn(ctx: *mut AVFormatContext, device_list: *mut c_void) -> c_int,
	pub create_device_capabilities: extern fn(ctx: *mut AVFormatContext, caps: *mut c_void) -> c_int,
	pub free_device_capabilities: extern fn(ctx: *mut AVFormatContext, caps: *mut c_void) -> c_int,

	pub data_codec: AVCodecID,
}

#[repr(C)]
pub struct AVInputFormat {
	pub name:      *const c_char,
	pub long_name: *const c_char,

	pub flags: c_int,
	pub extensions: *const c_char,
	pub codec_tag: *const *const AVCodecTag,
	pub priv_class: *const AVClass,
	pub mime_type: *const c_char,
	pub next: *mut AVInputFormat,
	pub raw_codec_id: c_int,
	pub priv_data_size: c_int,

	pub read_probe:     extern fn(data: *mut AVProbeData) -> c_int,
	pub read_header:    extern fn(ctx: *mut AVFormatContext) -> c_int,
	pub read_packet:    extern fn(ctx: *mut AVFormatContext, pkt: *mut AVPacket) -> c_int,
	pub read_close:     extern fn(ctx: *mut AVFormatContext) -> c_int,
	pub read_seek:      extern fn(ctx: *mut AVFormatContext, stream_index: c_int, timestamp: int64_t, flags: c_int) -> c_int,
	pub read_timestamp: extern fn(ctx: *mut AVFormatContext, stream_index: c_int, pos: *mut int64_t, pos_limit: int64_t) -> int64_t,
	pub read_play:      extern fn(ctx: *mut AVFormatContext) -> c_int,
	pub read_pause:     extern fn(ctx: *mut AVFormatContext) -> c_int,
	pub read_seek2:     extern fn(ctx: *mut AVFormatContext, stream_index: c_int, min_ts: int64_t, ts: int64_t, max_ts: int64_t, flags: c_int) -> c_int,

	pub get_device_list: extern fn(ctx: *mut AVFormatContext, device_list: *mut c_void) -> c_int,
	pub create_device_capabilities: extern fn(ctx: *mut AVFormatContext, caps: *mut c_void) -> c_int,
	pub free_device_capabilities: extern fn(ctx: *mut AVFormatContext, caps: *mut c_void) -> c_int,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVStreamParseType {
	AVSTREAM_PARSE_NONE,
	AVSTREAM_PARSE_FULL,
	AVSTREAM_PARSE_HEADERS,
	AVSTREAM_PARSE_TIMESTAMPS,
	AVSTREAM_PARSE_FULL_ONCE,
	AVSTREAM_PARSE_FULL_RAW = MKTAG!(0, b'R', b'A', b'W'),
}

#[derive(Debug)]
#[repr(C)]
pub struct AVIndexEntry {
	pub pos: int64_t,
	pub timestamp: int64_t,
	data: i32,
	pub min_distance: c_int,
}

impl AVIndexEntry {
	pub fn flags(&self) -> c_int {
		self.data >> 30
	}

	pub fn size(&self) -> c_int {
		self.data & 0b00111111111111111111111111111111
	}
}

pub const AVINDEX_KEYFRAME: c_int = 0x0001;

pub const AV_DISPOSITION_DEFAULT:          c_int = 0x0001;
pub const AV_DISPOSITION_DUB:              c_int = 0x0002;
pub const AV_DISPOSITION_ORIGINAL:         c_int = 0x0004;
pub const AV_DISPOSITION_COMMENT:          c_int = 0x0008;
pub const AV_DISPOSITION_LYRICS:           c_int = 0x0010;
pub const AV_DISPOSITION_KARAOKE:          c_int = 0x0020;
pub const AV_DISPOSITION_FORCED:           c_int = 0x0040;
pub const AV_DISPOSITION_HEARING_IMPAIRED: c_int = 0x0080;
pub const AV_DISPOSITION_VISUAL_IMPAIRED:  c_int = 0x0100;
pub const AV_DISPOSITION_CLEAN_EFFECTS:    c_int = 0x0200;
pub const AV_DISPOSITION_ATTACHED_PIC:     c_int = 0x0400;
pub const AV_DISPOSITION_CAPTIONS:         c_int = 0x10000;
pub const AV_DISPOSITION_DESCRIPTIONS:     c_int = 0x20000;
pub const AV_DISPOSITION_METADATA:         c_int = 0x40000;

pub const AV_PTS_WRAP_IGNORE:     c_int = 0;
pub const AV_PTS_WRAP_ADD_OFFSET: c_int = 1;
pub const AV_PTS_WRAP_SUB_OFFSET: c_int = -1;

pub const AVSTREAM_EVENT_FLAG_METADATA_UPDATED: c_int = 0x0001;

pub const MAX_STD_TIMEBASES: usize = 30 * 12 + 7 + 6;
pub const MAX_PROBE_PACKETS: usize = 2500;
pub const MAX_REORDER_DELAY: usize = 16;

#[repr(C)]
pub struct AVStreamInfo {
	pub last_dts: int64_t,
	pub duration_gcd: int64_t,
	pub duration_count: c_int,
	pub rfps_duration_sum: int64_t,
	pub duration_error: [[*mut c_double; MAX_STD_TIMEBASES]; 2],
	pub codec_info_duration: int64_t,
	pub codec_info_duration_fields: int64_t,
	pub found_decoder: c_int,
	pub last_duration: int64_t,
	pub fps_first_dts: int64_t,
	pub fps_first_dts_idx: c_int,
	pub fps_last_dts: int64_t,
	pub fps_last_dts_idx: c_int,
}

#[repr(C)]
pub struct AVStream {
	pub index: c_int,
	pub id: c_int,
	pub codec: *mut AVCodecContext,
	pub priv_data: *mut c_void,
	#[cfg(feature = "ff_api_lavf_frac")]
	pub pts: AVFrac,
	pub time_base: AVRational,
	pub start_time: int64_t,
	pub duration: int64_t,
	pub nb_frames: int64_t,
	pub disposition: c_int,
	pub discard: AVDiscard,
	pub sample_aspect_ratio: AVRational,
	pub metadata: *mut AVDictionary,
	pub avg_frame_rate: AVRational,
	pub attached_pic: AVPacket,
	pub side_data: *mut AVPacketSideData,
	pub nb_side_data: c_int,
	pub event_flags: c_int,
	pub info: *mut AVStreamInfo,
	pub pts_wrap_bits: c_int,
	pub first_dts: int64_t,
	pub cur_dts: int64_t,
	pub last_IP_pts: int64_t,
	pub last_IP_duration: c_int,
	pub probe_packets: c_int,
	pub codec_info_nb_frames: c_int,
	pub need_parsing: AVStreamParseType,
	pub parser: *mut AVCodecParserContext,
	pub last_in_packet_buffer: *mut AVPacketList,
	pub probe_data: AVProbeData,
	pub pts_buffer: [int64_t; MAX_REORDER_DELAY + 1],
	pub index_entries: *mut AVIndexEntry,
	pub nb_index_entries: c_int,
	pub index_entries_allocated_size: c_uint,
	pub r_frame_rate: AVRational,
	pub stream_identifiers: c_int,
	pub interleaver_chunk_size: int64_t,
	pub interleaver_chunk_duration: int64_t,
	pub request_probe: c_int,
	pub skip_to_keyframe: c_int,
	pub skip_samples: c_int,
	pub start_skip_samples: int64_t,
	pub first_discard_sample: int64_t,
	pub last_discard_sample: int64_t,
	pub nb_decoded_frames: c_int,
	pub mux_ts_offset: int64_t,
	pub pts_wrap_reference: int64_t,
	pub pts_wrap_behavior: c_int,
	pub update_initial_durations_done: c_int,
	pub pts_reorder_error: [int64_t; MAX_REORDER_DELAY + 1],
	pub pts_reorder_error_count: [uint8_t; MAX_REORDER_DELAY + 1],
	pub last_dts_for_order_check: int64_t,
	pub dts_ordered: uint8_t,
	pub dts_misordered: uint8_t,
	pub inject_global_side_data: c_int,
	pub recommended_encoder_configuration: *mut c_char,
	pub display_aspect_ratio: AVRational,
	pub priv_pts: *mut c_void,
}

pub const AV_PROGRAM_RUNNING: c_int = 1;

#[derive(Debug)]
#[repr(C)]
pub struct AVProgram {
	pub id: c_int,
	pub flags: c_int,
	pub discard: AVDiscard,
	pub stream_index: *mut c_uint,
	pub nb_stream_indexes: c_uint,
	pub metadata: *mut AVDictionary,
	pub program_num: c_int,
	pub pmt_pid: c_int,
	pub pcr_pid: c_int,

	pub start_time: int64_t,
	pub end_time: int64_t,
	pub pts_wrap_reference: int64_t,
	pub pts_wrap_behavior: c_int,
}

pub const AVFMTCTX_NOHEADER: c_int = 0x0001;

#[derive(Debug)]
#[repr(C)]
pub struct AVChapter {
	pub id: c_int,
	pub time_base: AVRational,
	pub start: int64_t,
	pub end: int64_t,
	pub metadata: *mut AVDictionary,
}

pub type av_format_control_message = extern fn(s: *mut AVFormatContext, kind: c_int, data: *mut c_void, data_size: size_t) -> c_int;
pub type AVOpenCallback = extern fn(*mut AVFormatContext, *mut *mut AVIOContext, *const c_char, c_int, *const AVIOInterruptCB, *mut *mut AVDictionary) -> c_int;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVDurationEstimationMethod {
	AVFMT_DURATION_FROM_PTS,
	AVFMT_DURATION_FROM_STREAM,
	AVFMT_DURATION_FROM_BITRATE,
}

pub type AVFormatInternal = c_void;

#[repr(C)]
pub struct AVFormatContext {
	pub av_class: *const AVClass,

	pub iformat: *mut AVInputFormat,
	pub oformat: *mut AVOutputFormat,

	pub priv_data: *mut c_void,

	pub pb: *mut AVIOContext,

	pub ctx_flags: c_int,

	pub nb_streams: c_uint,
	pub streams:    *mut *mut AVStream,

	pub filename: [c_char; 1024],

	pub start_time: int64_t,
	pub duration:   int64_t,

	pub bit_rate:    c_int,
	pub packet_size: c_uint,
	pub max_delay:   c_int,
	pub flags:       c_int,

	#[cfg(feature = "ff_api_probesize_32")]
	pub probesize: c_uint,
	#[cfg(feature = "ff_api_probesize_32")]
	pub max_analyze_duration: c_int,

	pub key:    *const uint8_t,
	pub keylen: c_int,

	pub nb_programs: c_uint,
	pub programs:    *mut *mut AVProgram,

	pub video_codec_id:    AVCodecID,
	pub audio_codec_id:    AVCodecID,
	pub subtitle_codec_id: AVCodecID,

	pub max_index_size:     c_uint,
	pub max_picture_buffer: c_uint,

	pub nb_chapters: c_uint,
	pub chapters:    *mut *mut AVChapter,

	pub metadata: *mut AVDictionary,

	pub start_time_realtime: int64_t,
	pub fps_probe_size: c_int,
	pub error_recognition: c_int,

	pub interrupt_callback: AVIOInterruptCB,

	pub debug: c_int,

	pub max_interleave_delta: int64_t,
	pub strict_std_compliance: c_int,
	pub event_flags: c_int,
	pub max_ts_probe: c_int,
	pub avoid_negative_ts: c_int,
	pub ts_id: c_int,
	pub audio_preload: c_int,

	pub max_chunk_duration: c_int,
	pub max_chunk_size: c_int,

	pub use_wallclock_as_timestamp: c_int,
	pub avio_flags: c_int,
	pub duration_estimation_method: AVDurationEstimationMethod,
	pub skip_initial_bytes: int64_t,
	pub correct_ts_overflow: c_uint,
	pub seek2any: c_int,
	pub flush_packets: c_int,
	pub probe_score: c_int,
	pub format_probesize: c_int,

	pub codec_whitelist:  *mut c_char,
	pub format_whitelist: *mut c_char,

	pub internal: *mut AVFormatInternal,

	pub io_repositioned: c_int,

	pub video_codec:    *mut AVCodec,
	pub audio_codec:    *mut AVCodec,
	pub subtitle_codec: *mut AVCodec,
	pub data_codec:     *mut AVCodec,

	pub metadata_header_padding: c_int,
	pub opaque: *mut c_void,

	pub control_message_cb: av_format_control_message,

	pub output_ts_offset: int64_t,
	#[cfg(feature = "ff_api_probesize_32")]
	pub max_analyze_duration2: int64_t,
	#[cfg(feature = "ff_api_probesize_32")]
	pub probesize2: int64_t,

	#[cfg(not(feature = "ff_api_probesize_32"))]
	pub max_analyze_duration: int64_t,
	#[cfg(not(feature = "ff_api_probesize_32"))]
	pub probesize: int64_t,

	pub dump_separator: *mut uint8_t,
	pub data_codec_id: AVCodecID,

	pub open_cb: extern fn(*mut AVFormatContext, *mut *mut AVIOContext, *const c_char, c_int, *const AVIOInterruptCB, *mut *mut AVDictionary) -> c_int,
}

pub const AVFMT_FLAG_GENPTS:          c_int = 0x0001;
pub const AVFMT_FLAG_IGNIDX:          c_int = 0x0002;
pub const AVFMT_FLAG_NONBLOCK:        c_int = 0x0004;
pub const AVFMT_FLAG_IGNDTS:          c_int = 0x0008;
pub const AVFMT_FLAG_NOFILLIN:        c_int = 0x0010;
pub const AVFMT_FLAG_NOPARSE:         c_int = 0x0020;
pub const AVFMT_FLAG_NOBUFFER:        c_int = 0x0040;
pub const AVFMT_FLAG_CUSTOM_IO:       c_int = 0x0080;
pub const AVFMT_FLAG_DISCARD_CORRUPT: c_int = 0x0100;
pub const AVFMT_FLAG_FLUSH_PACKETS:   c_int = 0x0200;
pub const AVFMT_FLAG_BITEXACT:        c_int = 0x0400;
pub const AVFMT_FLAG_MP4A_LATM:       c_int = 0x8000;
pub const AVFMT_FLAG_SORT_DTS:        c_int = 0x10000;
pub const AVFMT_FLAG_PRIV_OPT:        c_int = 0x20000;
pub const AVFMT_FLAG_KEEP_SIDE_DATA:  c_int = 0x40000;
pub const AVFMT_FLAG_FAST_SEEK:       c_int = 0x80000;

pub const FF_FDEBUG_TS: c_int = 0x0001;

pub const AVFMT_EVENT_FLAG_METADATA_UPDATED: c_int = 0x0001;

pub const AVFMT_AVOID_NEG_TS_AUTO:              c_int = -1;
pub const AVFMT_AVOID_NEG_TS_MAKE_NON_NEGATIVE: c_int = 1;
pub const AVFMT_AVOID_NEG_TS_MAKE_ZERO:         c_int = 2;

#[repr(C)]
pub struct AVPacketList {
	pub pkt: AVPacket,
	pub next: *mut AVPacketList,
}

pub const AVSEEK_FLAG_BACKWARD: c_int = 1;
pub const AVSEEK_FLAG_BYTE:     c_int = 2;
pub const AVSEEK_FLAG_ANY:      c_int = 4;
pub const AVSEEK_FLAG_FRAME:    c_int = 8;

extern {
	pub fn av_get_packet(s: *mut AVIOContext, pkt: *mut AVPacket, size: c_int) -> c_int;
	pub fn av_append_packet(s: *mut AVIOContext, pkt: *mut AVPacket, size: c_int) -> c_int;

	pub fn av_stream_get_r_frame_rate(s: *const AVStream) -> AVRational;
	pub fn av_stream_set_r_frame_rate(s: *mut AVStream, r: AVRational);

	pub fn av_stream_get_parser(s: *const AVStream) -> *mut AVCodecParserContext;

	pub fn av_stream_get_recommended_encoder_configuration(s: *const AVStream) -> *mut c_char;
	pub fn av_stream_set_recommended_encoder_configuration(s: *mut AVStream, configuration: *mut c_char);

	pub fn av_stream_get_end_pts(st: *const AVStream);

	pub fn av_format_get_probe_score(s: *const AVFormatContext) -> c_int;

	pub fn av_format_get_video_codec(s: *const AVFormatContext) -> *mut AVCodec;
	pub fn av_format_set_video_codec(s: *mut AVFormatContext, c: *mut AVCodec);

	pub fn av_format_get_audio_codec(s: *const AVFormatContext) -> *mut AVCodec;
	pub fn av_format_set_audio_codec(s: *mut AVFormatContext, c: *mut AVCodec);

	pub fn av_format_get_subtitle_codec(s: *const AVFormatContext) -> *mut AVCodec;
	pub fn av_format_set_subtitle_codec(s: *mut AVFormatContext, c: *mut AVCodec);

	pub fn av_format_get_data_codec(s: *const AVFormatContext) -> *mut AVCodec;
	pub fn av_format_set_data_codec(s: *mut AVFormatContext, c: *mut AVCodec);

	pub fn av_format_get_metadata_header_padding(s: *const AVFormatContext) -> c_int;
	pub fn av_format_set_metadata_header_padding(s: *mut AVFormatContext, c: c_int);

	pub fn av_format_get_opaque(s: *const AVFormatContext) -> *mut c_void;
	pub fn av_format_set_opaque(s: *mut AVFormatContext, opaque: *mut c_void);

	pub fn av_format_get_control_message_cb(s: *const AVFormatContext) -> av_format_control_message;
	pub fn av_format_set_control_message_cb(s: *mut AVFormatContext, callback: av_format_control_message);

	pub fn av_format_get_open_cb(s: *const AVFormatContext) -> AVOpenCallback;
	pub fn av_format_set_open_cb(s: *mut AVFormatContext, callback: AVOpenCallback);

	pub fn av_format_inject_global_side_data(s: *mut AVFormatContext);

	pub fn av_fmt_ctx_get_duration_estimation_method(ctx: *const AVFormatContext) -> AVDurationEstimationMethod;

	pub fn avformat_version() -> c_uint;
	pub fn avformat_configuration() -> *const c_char;
	pub fn avformat_license() -> *const c_char;

	pub fn av_register_all();
	pub fn av_register_input_format(format: *const AVInputFormat);
	pub fn av_register_output_format(format: *const AVOutputFormat);

	pub fn avformat_network_init() -> c_int;
	pub fn avformat_network_deinit() -> c_int;

	pub fn av_iformat_next(f: *const AVInputFormat) -> *mut AVInputFormat;
	pub fn av_oformat_next(f: *const AVOutputFormat) -> *mut AVOutputFormat;

	pub fn avformat_alloc_context() -> *mut AVFormatContext;
	pub fn avformat_free_context(s: *mut AVFormatContext);
	pub fn avformat_get_class() -> *const AVClass;

	pub fn avformat_new_stream(s: *mut AVFormatContext, c: *const AVCodec) -> *mut AVStream;
	pub fn av_stream_get_side_data(stream: *mut AVStream, kind: AVPacketSideData, size: *mut c_int) -> *mut uint8_t;

	pub fn av_new_program(s: *mut AVFormatContext, id: c_int) -> *mut AVProgram;

	pub fn avformat_alloc_output_context2(ctx: *mut *mut AVFormatContext, oformat: *const AVOutputFormat, format_name: *const c_char, filename: *const c_char) -> c_int;

	pub fn av_find_input_format(short_name: *const c_char) -> *mut AVInputFormat;
	pub fn av_probe_input_format(pd: *mut AVProbeData, is_opened: c_int) -> *mut AVInputFormat;
	pub fn av_probe_input_format2(pd: *mut AVProbeData, is_opened: c_int, score_max: *mut c_int) -> *mut AVInputFormat;
	pub fn av_probe_input_format3(pd: *mut AVProbeData, is_opened: c_int, score_ret: *mut c_int) -> *mut AVInputFormat;
	pub fn av_probe_input_buffer2(pb: *mut AVIOContext, fmt: *mut *mut AVInputFormat, filename: *const c_char, logctx: *mut c_void, offset: c_uint, max_probe_size: c_uint) -> c_int;
	pub fn av_probe_input_buffer(pb: *mut AVIOContext, fmt: *mut *mut AVInputFormat, filename: *const c_char, logctx: *mut c_void, offset: c_uint, max_probe_size: c_uint) -> c_int;

	pub fn avformat_open_input(ps: *mut *mut AVFormatContext, filename: *const c_char, fmt: *const AVInputFormat, options: *mut *mut AVDictionary) -> c_int;
	pub fn avformat_find_stream_info(ic: *mut AVFormatContext, options: *mut *mut AVDictionary) -> c_int;
	pub fn av_find_program_from_stream(ic: *const AVFormatContext, last: *mut AVProgram, s: c_int) -> *mut AVProgram;
	pub fn av_find_best_stream(ic: *const AVFormatContext, kind: AVMediaType, wanted_stream_nb: c_int, related_stream: c_int, decoder_ret: *mut *mut AVCodec, flags: c_int) -> c_int;

	pub fn av_read_frame(s: *mut AVFormatContext, pkt: *mut AVPacket) -> c_int;
	pub fn av_seek_frame(s: *mut AVFormatContext, stream_index: c_int, timestamp: int64_t, flags: c_int) -> c_int;
	pub fn avformat_seek_file(s: *mut AVFormatContext, stream_index: c_int, min_ts: int64_t, ts: int64_t, max_ts: int64_t, flags: c_int) -> c_int;
	pub fn avformat_flush(s: *mut AVFormatContext) -> c_int;
	pub fn av_read_play(s: *mut AVFormatContext) -> c_int;
	pub fn av_read_pause(s: *mut AVFormatContext) -> c_int;
	pub fn avformat_close_input(s: *mut *mut AVFormatContext);

	pub fn avformat_write_header(s: *mut AVFormatContext, options: *mut *mut AVDictionary) -> c_int;
	pub fn av_write_frame(s: *mut AVFormatContext, pkt: *const AVPacket) -> c_int;
	pub fn av_interleaved_write_frame(s: *mut AVFormatContext, pkt: *const AVPacket) -> c_int;
	pub fn av_write_uncoded_frame(s: *mut AVFormatContext, stream_index: c_int, frame: *mut AVFrame) -> c_int;
	pub fn av_interleaved_write_uncoded_frame(s: *mut AVFormatContext, stream_index: c_int, frame: *mut AVFrame) -> c_int;
	pub fn av_write_uncoded_frame_query(s: *mut AVFormatContext, stream_index: c_int) -> c_int;
	pub fn av_write_trailer(s: *mut AVFormatContext) -> c_int;

	pub fn av_guess_format(short_name: *const c_char, filename: *const c_char, mime_type: *const c_char) -> *mut AVOutputFormat;
	pub fn av_guess_codec(fmt: *const AVOutputFormat, short_name: *const c_char, filename: *const c_char, mime_type: *const c_char, kind: AVMediaType) -> AVCodecID;

	pub fn av_get_output_timestamp(s: *mut AVFormatContext, stream: c_int, dts: *mut int64_t, wall: *mut int64_t) -> c_int;

	pub fn av_hex_dump(f: *mut FILE, buf: *mut uint8_t, size: c_int);
	pub fn av_hex_dump_log(avcl: *mut c_void, level: c_int, buf: *const uint8_t, size: c_int);
	pub fn av_pkt_dump2(f: *mut FILE, pkt: *const AVPacket, dump_payload: c_int, st: *const AVStream);
	pub fn av_pkt_dump_log2(avcl: *mut c_void, level: c_int, pkt: *const AVPacket, dump_payload: c_int, st: *const AVStream);

	pub fn av_codec_get_id(tags: *const *const AVCodecTag, tag: c_uint) -> AVCodecID;
	pub fn av_codec_get_tag(tags: *const *const AVCodecTag, id: AVCodecID) -> c_uint;
	pub fn av_codec_get_tag2(tags: *const *const AVCodecTag, id: AVCodecID, tag: *mut c_uint) -> c_int;

	pub fn av_index_search_timestamp(st: *mut AVStream, timestamp: int64_t, flags: c_int) -> c_int;
	pub fn av_add_index_entry(st: *mut AVStream, pos: int64_t, timestamp: int64_t, size: c_int, distance: c_int, flags: c_int) -> c_int;
	pub fn av_url_split(proto: *mut c_char, proto_size: c_int, authorization: *mut c_char, authorization_size: c_int, hostname: *mut c_char, hostname_size: c_int, port_ptr: *mut c_int, path: *mut c_char, path_size: c_int, url: *const c_char);

	pub fn av_dump_format(ic: *const AVFormatContext, index: c_int, url: *const c_char, is_output: c_int);

	pub fn av_get_frame_filename(buf: *mut c_char, buf_size: c_int, path: *const c_char, number: c_int) -> c_int;
	pub fn av_filename_number_test(filename: *const c_char) -> c_int;
	pub fn av_sdp_create(ac: *mut *mut AVFormatContext, n_files: c_int, buf: *mut c_char, size: c_int) -> c_int;
	pub fn av_match_ext(filename: *const c_char, extensions: *const c_char) -> c_int;
	
	pub fn avformat_query_codec(ofmt: *const AVOutputFormat, codec_id: AVCodecID, std_compliance: c_int) -> c_int;
	pub fn avformat_get_riff_video_tags() -> *const AVCodecTag;
	pub fn avformat_get_riff_audio_tags() -> *const AVCodecTag;
	pub fn avformat_get_mov_video_tags() -> *const AVCodecTag;
	pub fn avformat_get_mov_audio_tags() -> *const AVCodecTag;

	pub fn av_guess_sample_aspect_ratio(format: *mut AVFormatContext, stream: *mut AVStream, frame: *mut AVFrame) -> AVRational;
	pub fn av_guess_frame_rate(ctx: *mut AVFormatContext, stream: *mut AVStream, frame: *mut AVStream) -> AVRational;

	pub fn avformat_match_stream_specifier(s: *mut AVFormatContext, st: *mut AVStream, spec: *const c_char) -> c_int;
	pub fn avformat_queue_attached_pictures(s: *mut AVFormatContext) -> c_int;
}
