use libc::c_int;

use crate::ffi::*;

bitflags! {
	#[cfg_attr(feature = "serde", derive(serde_derive::Serialize, serde_derive::Deserialize))]
	#[cfg_attr(feature = "serde", serde(crate = "serde_", rename_all = "kebab-case"))]
	pub struct Flags: c_int {
		const CODED_ORDER = SLICE_FLAG_CODED_ORDER;
		const ALLOW_FIELD = SLICE_FLAG_ALLOW_FIELD;
		const ALLOW_PLANE = SLICE_FLAG_ALLOW_PLANE;
	}
}
