use libc::uint32_t;

extern {
	pub fn av_get_random_seed() -> uint32_t;
}
