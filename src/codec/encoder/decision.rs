use libc::c_int;

use crate::ffi::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde_derive::Serialize, serde_derive::Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_", rename_all = "kebab-case"))]
pub enum Decision {
	Simple,
	Bits,
	RateDistortion,
}

impl From<c_int> for Decision {
	fn from(value: c_int) -> Decision {
		match value {
			FF_MB_DECISION_SIMPLE => Decision::Simple,
			FF_MB_DECISION_BITS => Decision::Bits,
			FF_MB_DECISION_RD => Decision::RateDistortion,

			_ => Decision::Simple,
		}
	}
}

impl Into<c_int> for Decision {
	fn into(self) -> c_int {
		match self {
			Decision::Simple => FF_MB_DECISION_SIMPLE,
			Decision::Bits => FF_MB_DECISION_BITS,
			Decision::RateDistortion => FF_MB_DECISION_RD,
		}
	}
}
