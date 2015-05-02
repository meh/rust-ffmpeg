use libc::c_char;

#[derive(Eq, PartialEq, Debug)]
#[repr(C)]
pub enum AVPictureType {
	AV_PICTURE_TYPE_NONE = 0,
	AV_PICTURE_TYPE_I,
	AV_PICTURE_TYPE_P,
	AV_PICTURE_TYPE_B,
	AV_PICTURE_TYPE_S,
	AV_PICTURE_TYPE_SI,
	AV_PICTURE_TYPE_SP,
	AV_PICTURE_TYPE_BI,
}

#[link(name = "avutil")]
extern {
	pub fn av_get_picture_type_char(pict_type: AVPictureType) -> c_char;
}
