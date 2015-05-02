use libc::{uint8_t, int16_t, int32_t, uint64_t};

#[derive(Debug)]
#[repr(C)]
pub struct AVMotionVector {
	source: int32_t,

	w: uint8_t,
	h: uint8_t,

	src_x: int16_t,
	src_y: int16_t,

	dst_x: int16_t,
	dst_y: int16_t,

	flags: uint64_t,
}
