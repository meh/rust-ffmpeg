use libc::{c_void, c_int, uint8_t};

pub type AVBuffer = c_void;

#[derive(Debug)]
#[repr(C)]
pub struct AVBufferRef {
	pub buffer: *mut AVBuffer,
	pub data:   *mut uint8_t,
	pub size:   c_int,
}

pub type AVBufferPool = c_void;

pub const AV_BUFFER_FLAG_READONLY: c_int = 1;

extern {
	pub fn av_buffer_alloc(size: c_int) -> *mut AVBufferRef;
	pub fn av_buffer_allocz(size: c_int) -> *mut AVBufferRef;
	pub fn av_buffer_create(data: *mut uint8_t, size: c_int, free: extern fn(*mut c_void, *mut uint8_t), opaque: *mut c_void, flags: c_int) -> *mut AVBufferRef;
	pub fn av_buffer_default_free(opaque: *mut c_void, data: *mut uint8_t);
	pub fn av_buffer_ref(buf: *mut AVBufferRef) -> *mut AVBufferRef;
	pub fn av_buffer_unref(buf: *mut *mut AVBufferRef);
	pub fn av_buffer_is_writable(buf: *const AVBufferRef) -> c_int;
	pub fn av_buffer_get_opaque(buf: *const AVBufferRef) -> *mut c_void;
	pub fn av_buffer_get_ref_count(buf: *const AVBufferRef) -> c_int;
	pub fn av_buffer_make_writable(buf: *mut *mut AVBufferRef) -> c_int;
	pub fn av_buffer_realloc(buf: *mut *mut AVBufferRef, size: c_int) -> c_int;

	pub fn av_buffer_pool_init(size: c_int, alloc: extern fn(c_int) -> *mut AVBufferRef) -> *mut AVBufferPool;
	pub fn av_buffer_pool_uninit(pool: *mut *mut AVBufferPool);
	pub fn av_buffer_pool_get(pool: *mut AVBufferPool) -> *mut AVBufferRef;
}
