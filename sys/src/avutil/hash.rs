use libc::{c_void, c_char, c_int, uint8_t};

pub type AVHashContext = c_void;

pub const AV_HASH_MAX_SIZE: c_int = 64;

extern {
	pub fn av_hash_alloc(ctx: *mut *mut AVHashContext, name: *const c_char) -> c_int;
	pub fn av_hash_names(i: c_int) -> *const c_char;
	pub fn av_hash_get_name(ctx: *const AVHashContext) -> *const c_char;
	pub fn av_hash_get_size(ctx: *const AVHashContext) -> c_int;
	pub fn av_hash_init(ctx: *mut AVHashContext);
	pub fn av_hash_update(ctx: *mut AVHashContext, src: *const uint8_t, len: c_int);
	pub fn av_hash_final(ctx: *mut AVHashContext, dst: *mut uint8_t);
	pub fn av_hash_final_bin(ctx: *mut AVHashContext, dst: *mut uint8_t, size: c_int);
	pub fn av_hash_final_hex(ctx: *mut AVHashContext, dst: *mut uint8_t, size: c_int);
	pub fn av_hash_final_b64(ctx: *mut AVHashContext, dst: *mut uint8_t, size: c_int);
	pub fn av_hash_freep(ctx: *mut *mut AVHashContext);
}
