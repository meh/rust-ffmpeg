use libc::{c_void, c_int, c_uint, uint8_t, uint32_t, size_t};

#[derive(Debug)]
#[repr(C)]
pub struct AVFifoBuffer {
	pub buffer: *mut uint8_t,

	pub rptr: *mut uint8_t,
	pub wptr: *mut uint8_t,
	pub end:  *mut uint8_t,

	pub rndx: uint32_t,
	pub wndx: uint32_t,
}

#[inline(always)]
pub unsafe fn av_fifo_peek2(f: *const AVFifoBuffer, offs: c_int) -> *mut uint8_t {
	let ptr = (*f).rptr.offset(offs as isize);

	if ptr > (*f).end {
		(*f).buffer.offset((ptr as usize - (*f).end as usize) as isize)
	}
	else if ptr < (*f).buffer {
		(*f).end.offset(-(((*f).buffer as usize - ptr as usize) as isize))
	}
	else {
		ptr
	}
}

extern {
	pub fn av_fifo_alloc(size: c_uint) -> *mut AVFifoBuffer;
	pub fn av_fifo_alloc_array(nmemb: size_t, size: size_t) -> *mut AVFifoBuffer;
	pub fn av_fifo_free(f: *mut AVFifoBuffer);
	pub fn av_fifo_freep(f: *mut *mut AVFifoBuffer);
	pub fn av_fifo_reset(f: *mut AVFifoBuffer);
	pub fn av_fifo_size(f: *const AVFifoBuffer) -> c_int;
	pub fn av_fifo_space(f: *const AVFifoBuffer) -> c_int;
	pub fn av_fifo_generic_peek(f: *mut AVFifoBuffer, dest: *mut c_void, buf_size: c_int, func: extern fn(*mut c_void, *mut c_void, c_int)) -> c_int;
	pub fn av_fifo_generic_read(f: *mut AVFifoBuffer, dest: *mut c_void, buf_size: c_int, func: extern fn(*mut c_void, *mut c_void, c_int)) -> c_int;
	pub fn av_fifo_generic_write(f: *mut AVFifoBuffer, src: *mut c_void, size: c_int, func: extern fn(*mut c_void, *mut c_void, c_int) -> c_int) -> c_int;
	pub fn av_fifo_realloc2(f: *mut AVFifoBuffer, size: c_uint) -> c_int;
	pub fn av_fifo_grow(f: *mut AVFifoBuffer, additional_space: c_int) -> c_int;
	pub fn av_fifo_drain(f: *mut AVFifoBuffer, size: c_int);
}
