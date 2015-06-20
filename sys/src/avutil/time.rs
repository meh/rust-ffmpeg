use libc::{c_int, c_uint, int64_t};

extern {
	pub fn av_gettime() -> int64_t;
	pub fn av_gettime_relative() -> int64_t;
	pub fn av_gettime_relative_is_monotonic() -> c_int;
	pub fn av_usleep(usec: c_uint) -> c_int;
}
