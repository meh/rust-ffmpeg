use libc::{int32_t, uint32_t};

#[derive(Debug)]
#[repr(C)]
pub struct AVReplayGain {
	pub track_gain: int32_t,
	pub track_peak: uint32_t,
	pub album_gain: int32_t,
	pub album_peak: uint32_t,
}
