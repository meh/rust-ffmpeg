use libc::{int32_t, uint32_t};

#[derive(Debug)]
#[repr(C)]
pub struct AVReplayGain {
	track_gain: int32_t,
	track_peak: uint32_t,
	album_gain: int32_t,
	album_peak: uint32_t,
}
