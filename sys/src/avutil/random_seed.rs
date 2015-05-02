use libc::uint32_t;

#[link(name = "avutil")]
extern {
	pub fn av_get_random_seed() -> uint32_t;
}
