use libc::c_char;

#[repr(C)]
pub enum AVMediaType {
	AVMEDIA_TYPE_UNKNOWN = -1,
	AVMEDIA_TYPE_VIDEO,
	AVMEDIA_TYPE_AUDIO,
	AVMEDIA_TYPE_DATA,
	AVMEDIA_TYPE_SUBTITLE,
	AVMEDIA_TYPE_ATTACHMENT,
	AVMEDIA_TYPE_NB
}

#[link(name = "avutil")]
extern {
	pub fn av_get_media_type_string(media_type: AVMediaType) -> *const c_char;
}

