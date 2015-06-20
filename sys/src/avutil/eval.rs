use libc::{c_void, c_char, c_int, c_double};

pub type AVExpr = c_void;

extern {
	pub fn av_expr_parse_and_eval(res: *mut c_double, s: *const c_char,
	                              const_names: *const *const c_char, const_values: *const c_double,
	                              func1_names: *const *const c_char, funcs1: *const extern fn(*mut c_void, c_double) -> c_double,
	                              func2_names: *const *const c_char, funcs2: *const extern fn(*mut c_void, c_double, c_double) -> c_double,
	                              opaque: *mut c_void, log_offset: c_int, log_ctx: *mut c_void) -> c_int;

	pub fn av_expr_parse(expr: *mut *mut AVExpr, s: *const c_char,
	                     const_names: *const *const c_char, const_values: *const c_double,
	                     func1_names: *const *const c_char, funcs1: *const extern fn(*mut c_void, c_double) -> c_double,
	                     func2_names: *const *const c_char, funcs2: *const extern fn(*mut c_void, c_double, c_double) -> c_double,
	                     log_offset: c_int, log_ctx: *mut c_void) -> c_int;

	pub fn av_expr_eval(e: *mut AVExpr, const_values: *const c_double, opaque: *mut c_void) -> c_double;

	pub fn av_expr_free(e: *mut AVExpr);

	pub fn av_strtod(numstr: *const c_char, tail: *mut *mut c_char) -> c_double;
}
