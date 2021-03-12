use libc::c_int;

use crate::ffi::*;

bitflags! {
	#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
	#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
	pub struct Flags: c_int {
		const FORCE = SWR_FLAG_RESAMPLE;
	}
}
