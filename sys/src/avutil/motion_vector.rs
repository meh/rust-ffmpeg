use libc::{uint8_t, int16_t, int32_t, uint64_t};

#[derive(Debug)]
#[repr(C)]
pub struct AVMotionVector {
	pub source: int32_t,

	pub w: uint8_t,
	pub h: uint8_t,

	pub src_x: int16_t,
	pub src_y: int16_t,

	pub dst_x: int16_t,
	pub dst_y: int16_t,

	pub flags: uint64_t,
}
