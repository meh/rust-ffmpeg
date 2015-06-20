use libc::{c_void, c_int, uint8_t, uint64_t};

pub type AVMurMur3 = c_void;

extern {
	pub fn av_murmur3_alloc() -> *mut AVMurMur3;
	pub fn av_murmur3_init_seeded(c: *mut AVMurMur3, seed: uint64_t);
	pub fn av_murmur3_init(c: *mut AVMurMur3);
	pub fn av_murmur3_update(c: *mut AVMurMur3, src: *const uint8_t, len: c_int);
	pub fn av_murmur3_final(c: *mut AVMurMur3, dst: *mut uint8_t);
}
