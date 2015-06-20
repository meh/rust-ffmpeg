use libc::{c_int, int32_t, c_double};

extern {
	pub fn av_display_rotation_get(matrix: *const int32_t) -> c_double;
	pub fn av_display_rotation_set(matrix: *mut int32_t, angle: c_double);
	pub fn av_display_matrix_flip(matrix: *mut int32_t, hflip: c_int, vflip: c_int);
}
