use libc::{c_int};
use super::frame::AVFrame;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVStereo3DType {
	AV_STEREO3D_2D,
	AV_STEREO3D_SIDEBYSIDE,
	AV_STEREO3D_TOPBOTTOM,
	AV_STEREO3D_FRAMESEQUENCE,
	AV_STEREO3D_CHECKERBOARD,
	AV_STEREO3D_SIDEBYSIDE_QUINCUNX,
	AV_STEREO3D_LINES,
	AV_STEREO3D_COLUMNS,
}

pub const AV_STEREO3D_FLAG_INVERT: c_int = 1 << 0;

#[derive(Debug)]
#[repr(C)]
pub struct AVStereo3D {
	pub kind:  AVStereo3DType,
	pub flags: c_int,
}

extern {
	pub fn av_stereo3d_alloc() -> *mut AVStereo3D;
	pub fn av_stereo3d_create_side_data(frame: *mut AVFrame) -> *mut AVStereo3D;
}
