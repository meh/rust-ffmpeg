use libc::{c_double};
use super::frame::AVFrame;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVDownmixType {
	AV_DOWNMIX_TYPE_UNKNOWN,
	AV_DOWNMIX_TYPE_LORO,
	AV_DOWNMIX_TYPE_LTRT,
	AV_DOWNMIX_TYPE_DPLII,
	AV_DOWNMIX_TYPE_NB,
}

#[derive(Debug)]
#[repr(C)]
pub struct AVDownmixInfo {
	pub preferred_downmix_type: AVDownmixType,

	pub center_mix_level: c_double,
	pub center_mix_level_ltrt: c_double,

	pub surround_mix_level: c_double,
	pub surround_mix_level_ltrt: c_double,

	pub lfe_mix_level: c_double,
}

extern {
	pub fn av_downmix_info_update_side_data(frame: *mut AVFrame) -> *mut AVDownmixInfo;
}
