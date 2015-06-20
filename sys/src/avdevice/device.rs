use libc::{c_void, c_char, c_int, c_uint, int64_t, size_t};
use super::super::avutil::{AVClass, AVPixelFormat, AVSampleFormat, AVOption, AVDictionary, AVRational};
use super::super::avcodec::{AVCodecID};
use super::super::avformat::{AVFormatContext, AVInputFormat, AVOutputFormat};

#[derive(Debug)]
#[repr(C)]
pub struct AVDeviceRect {
	pub x: c_int,
	pub y: c_int,
	pub width: c_int,
	pub height: c_int,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVAppToDevMessageType {
	AV_APP_TO_DEV_NONE           = MKBETAG!(b'N', b'O', b'N', b'E'),
	AV_APP_TO_DEV_WINDOW_SIZE    = MKBETAG!(b'G', b'E', b'O', b'M'),
	AV_APP_TO_DEV_WINDOW_REPAINT = MKBETAG!(b'R', b'E', b'P', b'A'),
	AV_APP_TO_DEV_PAUSE          = MKBETAG!(b'P', b'A', b'U', b' '),
	AV_APP_TO_DEV_PLAY           = MKBETAG!(b'P', b'L', b'A', b'Y'),
	AV_APP_TO_DEV_TOGGLE_PAUSE   = MKBETAG!(b'P', b'A', b'U', b'T'),
	AV_APP_TO_DEV_SET_VOLUME     = MKBETAG!(b'S', b'V', b'O', b'L'),
	AV_APP_TO_DEV_MUTE           = MKBETAG!(b' ', b'M', b'U', b'T'),
	AV_APP_TO_DEV_UNMUTE         = MKBETAG!(b'U', b'M', b'U', b'T'),
	AV_APP_TO_DEV_TOGGLE_MUTE    = MKBETAG!(b'T', b'M', b'U', b'T'),
	AV_APP_TO_DEV_GET_VOLUME     = MKBETAG!(b'G', b'V', b'O', b'L'),
	AV_APP_TO_DEV_GET_MUTE       = MKBETAG!(b'G', b'M', b'U', b'T'),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVDevToAppMessageType {
	AV_DEV_TO_APP_NONE                  = MKBETAG!(b'N', b'O', b'N', b'E'),
	AV_DEV_TO_APP_CREATE_WINDOW_BUFFER  = MKBETAG!(b'B', b'C', b'R', b'E'),
	AV_DEV_TO_APP_PREPARE_WINDOW_BUFFER = MKBETAG!(b'B', b'P', b'R', b'E'),
	AV_DEV_TO_APP_DISPLAY_WINDOW_BUFFER = MKBETAG!(b'B', b'D', b'I', b'S'),
	AV_DEV_TO_APP_DESTROY_WINDOW_BUFFER = MKBETAG!(b'B', b'D', b'E', b'S'),
	AV_DEV_TO_APP_BUFFER_OVERFLOW       = MKBETAG!(b'B', b'O', b'F', b'L'),
	AV_DEV_TO_APP_BUFFER_UNDERFLOW      = MKBETAG!(b'B', b'U', b'F', b'L'),
	AV_DEV_TO_APP_BUFFER_READABLE       = MKBETAG!(b'B', b'R', b'D', b' '),
	AV_DEV_TO_APP_BUFFER_WRITABLE       = MKBETAG!(b'B', b'W', b'R', b' '),
	AV_DEV_TO_APP_MUTE_STATE_CHANGED    = MKBETAG!(b'C', b'M', b'U', b'T'),
	AV_DEV_TO_APP_VOLUME_LEVEL_CHANGED  = MKBETAG!(b'C', b'V', b'O', b'L'),
}

#[derive(Debug)]
#[repr(C)]
pub struct AVDeviceCapabilitiesQuery {
	pub av_class: *const AVClass,
	pub device_context: *const AVFormatContext,
	pub codec: AVCodecID,
	pub sample_format: AVSampleFormat,
	pub pixel_format: AVPixelFormat,
	pub sample_rate: c_int,
	pub channels: c_int,
	pub channel_layout: int64_t,
	pub window_width: c_int,
	pub window_height: c_int,
	pub frame_width: c_int,
	pub frame_height: c_int,
	pub fps: AVRational,
}

#[derive(Debug)]
#[repr(C)]
pub struct AVDeviceInfo {
	pub device_name: *mut c_char,
	pub device_description: *mut c_char,
}

#[derive(Debug)]
#[repr(C)]
pub struct AVDeviceInfoList {
	pub devices: *mut *mut AVDeviceInfo,
	pub nb_devices: c_int,
	pub default_device: c_int,
}

extern {
	pub static av_device_capabilities: *const AVOption;

	pub fn avdevice_version() -> c_uint;
	pub fn avdevice_configuration() -> *const c_char;
	pub fn avdevice_license() -> *const c_char;

	pub fn avdevice_register_all();

	pub fn av_input_audio_device_next(d: *mut AVInputFormat) -> *mut AVInputFormat;
	pub fn av_input_video_device_next(d: *mut AVInputFormat) -> *mut AVInputFormat;

	pub fn av_output_audio_device_next(d: *mut AVOutputFormat) -> *mut AVOutputFormat;
	pub fn av_output_video_device_next(d: *mut AVOutputFormat) -> *mut AVOutputFormat;

	pub fn avdevice_app_to_dev_control_message(s: *const AVFormatContext, kind: AVAppToDevMessageType, data: *mut c_void, data_size: size_t) -> c_int;
	pub fn avdevice_dev_to_app_control_message(s: *const AVFormatContext, kind: AVDevToAppMessageType, data: *mut c_void, data_size: size_t) -> c_int;

	pub fn avdevice_capabilities_create(caps: *mut *mut AVDeviceCapabilitiesQuery, s: *const AVFormatContext, device_options: *mut *mut AVDictionary) -> c_int;
	pub fn avdevice_capabilities_free(caps: *mut *mut AVDeviceCapabilitiesQuery, s: *const AVFormatContext);

	pub fn avdevice_list_devices(s: *const AVFormatContext, device_list: *mut *mut AVDeviceInfoList) -> c_int;
	pub fn avdevice_free_list_devices(device_list: *mut *mut AVDeviceInfoList);
	pub fn avdevice_list_input_sources(device: *mut AVInputFormat, device_name: *const c_char, device_options: *mut AVDictionary, device_list: *mut *mut AVDeviceInfoList) -> c_int;
	pub fn avdevice_list_output_sinks(device: *mut AVOutputFormat, device_name: *const c_char, device_options: *mut AVDictionary, device_list: *mut *mut AVDeviceInfoList) -> c_int;
}
