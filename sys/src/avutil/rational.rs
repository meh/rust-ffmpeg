use libc::{c_int, uint32_t, int64_t, c_double};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub struct AVRational {
	pub num: c_int,
	pub den: c_int,
}

#[inline(always)]
pub unsafe fn av_make_q(num: c_int, den: c_int) -> AVRational {
	AVRational { num: num, den: den }
}

#[inline(always)]
pub unsafe fn av_cmp_q(a: AVRational, b: AVRational) -> c_int {
	let tmp: int64_t = a.num as int64_t * b.den as int64_t - b.num as int64_t * a.den as int64_t;

	if tmp != 0 {
		(((tmp ^ a.den as int64_t ^ b.den as int64_t) >> 63) | 1) as c_int
	}
	else if b.den != 0 && a.den != 0 {
		0
	}
	else if a.num != 0 && b.num != 0 {
		((a.num as int64_t >> 31) - (b.num as int64_t >> 31)) as c_int
	}
	else {
		c_int::min_value()
	}
}

#[inline(always)]
pub unsafe fn av_q2d(a: AVRational) -> c_double {
	a.num as c_double / a.den as c_double
}

#[inline(always)]
pub unsafe fn av_inv_q(q: AVRational) -> AVRational {
	AVRational { num: q.den, den: q.num }
}

extern {
	pub fn av_reduce(dst_num: *mut c_int, dst_den: *mut c_int, num: int64_t, den: int64_t, max: int64_t) -> c_int;
	pub fn av_mul_q(b: AVRational, c: AVRational) -> AVRational;
	pub fn av_div_q(b: AVRational, c: AVRational) -> AVRational;
	pub fn av_add_q(b: AVRational, c: AVRational) -> AVRational;
	pub fn av_sub_q(b: AVRational, c: AVRational) -> AVRational;
	pub fn av_d2q(d: c_double, max: c_int) -> AVRational;
	pub fn av_nearer_q(q: AVRational, q1: AVRational, q2: AVRational) -> c_int;
	pub fn av_find_nearest_q_idx(q: AVRational, q_list: *const AVRational) -> c_int;
	pub fn av_q2intfloat(q: AVRational) -> uint32_t;
}
