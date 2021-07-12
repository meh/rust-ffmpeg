use libc::c_int;

use crate::ffi::*;

bitflags! {
	#[cfg_attr(feature = "serde", derive(serde_derive::Serialize, serde_derive::Deserialize))]
	#[cfg_attr(feature = "serde", serde(crate = "serde_", rename_all = "kebab-case"))]
	pub struct Conceal: c_int {
		const GUESS_MVS   = FF_EC_GUESS_MVS;
		const DEBLOCK     = FF_EC_DEBLOCK;
		const FAVOR_INTER = FF_EC_FAVOR_INTER;
	}
}
