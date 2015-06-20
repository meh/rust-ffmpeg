use libc::{c_int, int64_t, uint64_t};
use super::rational::AVRational;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVRounding {
	AV_ROUND_ZERO        = 0,
	AV_ROUND_INF         = 1,
	AV_ROUND_DOWN        = 2,
	AV_ROUND_UP          = 3,
	AV_ROUND_NEAR_INF    = 5,
	AV_ROUND_PASS_MINMAX = 8192,
}

extern {
	pub fn av_gcd(a: int64_t, b: int64_t) -> int64_t;
	pub fn av_rescale(a: int64_t, b: int64_t, c: int64_t) -> int64_t;
	pub fn av_rescale_rnd(a: int64_t, b: int64_t, c: int64_t, r: AVRounding) -> int64_t;
	pub fn av_rescale_q(a: int64_t, bq: AVRational, cq: AVRational) -> int64_t;
	pub fn av_rescale_q_rnd(a: int64_t, bq: AVRational, cq: AVRational, r: AVRounding) -> int64_t;
	pub fn av_compare_ts(ts_a: int64_t, tb_a: AVRational, ts_b: int64_t, tb_b: AVRational) -> c_int;
	pub fn av_compare_mod(a: uint64_t, b: uint64_t, m: uint64_t) -> int64_t;
	pub fn av_rescale_delta(in_tb: AVRational, in_ts: int64_t, fs_tb: AVRational, duration: c_int, last: *mut int64_t, out_tb: AVRational) -> int64_t;
	pub fn av_add_stable(ts_tb: AVRational, ts: int64_t, inc_tb: AVRational, inc: int64_t) -> int64_t;
}
