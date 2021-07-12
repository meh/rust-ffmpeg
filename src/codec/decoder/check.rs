use libc::c_int;

use crate::ffi::*;

bitflags! {
	#[cfg_attr(feature = "serde", derive(serde_derive::Serialize, serde_derive::Deserialize))]
	#[cfg_attr(feature = "serde", serde(crate = "serde_", rename_all = "kebab-case"))]
	pub struct Check: c_int {
		const CRC      = AV_EF_CRCCHECK;
		const BISTREAM = AV_EF_BITSTREAM;
		const BUFFER   = AV_EF_BUFFER;
		const EXPLODE  = AV_EF_EXPLODE;

		const IGNORE_ERROR = AV_EF_IGNORE_ERR;
		const CAREFUL      = AV_EF_CAREFUL;
		const COMPLIANT    = AV_EF_COMPLIANT;
		const AGGRESSIVE   = AV_EF_AGGRESSIVE;
	}
}
