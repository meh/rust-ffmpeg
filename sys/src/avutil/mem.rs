use libc::{c_void, c_char, c_int, c_uint, uint8_t, size_t};
use std::ptr;
//use std::mem;
//use super::error::AVERROR;

#[inline(always)]
pub unsafe fn av_malloc_array(nmemb: size_t, size: size_t) -> *mut c_void {
	if size == 0 || nmemb >= c_int::max_value() as size_t / size {
		ptr::null_mut()
	}
	else {
		av_malloc(nmemb * size)
	}
}

#[inline(always)]
pub unsafe fn av_mallocz_array(nmemb: size_t, size: size_t) -> *mut c_void {
	if size == 0 || nmemb >= c_int::max_value() as size_t / size {
		ptr::null_mut()
	}
	else {
		av_mallocz(nmemb * size)
	}
}

//#[inline(always)]
//pub unsafe fn av_size_mult(a: size_t, b: size_t, r: *mut size_t) -> c_int {
//	use libc::EINVAL;
//
//	let t = a * b;
//
//	if (a | b) >= (1 << (mem::size_of::<size_t>() * 4)) && a != 0 && t / a != b {
//		AVERROR(EINVAL)
//	}
//	else {
//		*r = t;
//
//		0
//	}
//}

extern {
	pub fn av_malloc(size: size_t) -> *mut c_void;
	pub fn av_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
	pub fn av_realloc_f(ptr: *mut c_void, nelem: size_t, elsize: size_t) -> *mut c_void;
	pub fn av_reallocp(ptr: *mut c_void, size: size_t) -> c_int;
	pub fn av_realloc_array(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
	pub fn av_reallocp_array(ptr: *mut c_void, nmemb: size_t, size: size_t) -> c_int;
	pub fn av_free(ptr: *mut c_void);
	pub fn av_mallocz(size: size_t) -> *mut c_void;
	pub fn av_calloc(nmemb: size_t, size: size_t) -> *mut c_void;
	pub fn av_strdup(s: *const c_char) -> *mut c_char;
	pub fn av_strndup(s: *const c_char, len: size_t) -> *mut c_char;
	pub fn av_memdup(p: *const c_void, size: size_t) -> *mut c_void;
	pub fn av_freep(ptr: *mut c_void);

	pub fn av_dynarray_add(tab_ptr: *mut c_void, nb_ptr: *mut c_int, elem: *mut c_void);
	pub fn av_dynarray_add_nofree(tab_ptr: *mut c_void, nb_ptr: *mut c_int, elem: *mut c_void) -> c_int;
	pub fn av_dynarray2_add(tab_ptr: *mut *mut c_void, nb_ptr: *mut c_int, elem_size: size_t, elem_data: *const uint8_t);
	
	pub fn av_max_alloc(max: size_t);
	pub fn av_memcpy_backptr(dst: *mut uint8_t, back: c_int, cnt: c_int);
	pub fn av_fast_realloc(ptr: *mut c_void, size: *mut c_uint, min_size: size_t) -> *mut c_void;
	pub fn av_fast_malloc(ptr: *mut c_void, size: *mut c_uint, min_size: size_t);
}
