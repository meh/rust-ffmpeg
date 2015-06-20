use libc::{c_void, c_int, uint8_t};

pub type AVMD5 = c_void;

extern {
	pub static av_md5_size: c_int;

	pub fn av_md5_alloc() -> *mut AVMD5;
	pub fn av_md5_init(ctx: *mut AVMD5);
	pub fn av_md5_update(ctx: *mut AVMD5, src: *const uint8_t, len: c_int);
	pub fn av_md5_final(ctx: *mut AVMD5, dst: *mut uint8_t);
	pub fn av_md5_sum(dst: *mut uint8_t, src: *const uint8_t, len: c_int);
}
