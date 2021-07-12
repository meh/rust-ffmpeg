use libc::c_int;

use crate::ffi::*;

bitflags! {
	#[cfg_attr(feature = "serde", derive(serde_derive::Serialize, serde_derive::Deserialize))]
	#[cfg_attr(feature = "serde", serde(crate = "serde_", rename_all = "kebab-case"))]
	pub struct Flags: c_int {
		const DYNAMIC_INPUTS            = AVFILTER_FLAG_DYNAMIC_INPUTS;
		const DYNAMIC_OUTPUTS           = AVFILTER_FLAG_DYNAMIC_OUTPUTS;
		const SLICE_THREADS             = AVFILTER_FLAG_SLICE_THREADS;
		const SUPPORT_TIMELINE_GENERIC  = AVFILTER_FLAG_SUPPORT_TIMELINE_GENERIC;
		const SUPPORT_TIMELINE_INTERNAL = AVFILTER_FLAG_SUPPORT_TIMELINE_INTERNAL;
		const SUPPORT_TIMELINE          = AVFILTER_FLAG_SUPPORT_TIMELINE;
	}
}
