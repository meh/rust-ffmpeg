use std::ffi::CStr;

use crate::ffi::{AVColorRange::*, *};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde_derive::Serialize, serde_derive::Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_", rename_all = "kebab-case"))]
pub enum Range {
	Unspecified,
	MPEG,
	JPEG,
}

impl Range {
	pub fn name(&self) -> Option<&'static str> {
		if *self == Range::Unspecified {
			return None;
		}
		unsafe {
			let ptr = av_color_range_name((*self).into());
			ptr
				.as_ref()
				.map(|ptr| std::str::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
		}
	}
}

impl From<AVColorRange> for Range {
	fn from(value: AVColorRange) -> Self {
		match value {
			AVCOL_RANGE_UNSPECIFIED => Range::Unspecified,
			AVCOL_RANGE_MPEG => Range::MPEG,
			AVCOL_RANGE_JPEG => Range::JPEG,
			AVCOL_RANGE_NB => Range::Unspecified,
		}
	}
}

impl Into<AVColorRange> for Range {
	fn into(self) -> AVColorRange {
		match self {
			Range::Unspecified => AVCOL_RANGE_UNSPECIFIED,
			Range::MPEG => AVCOL_RANGE_MPEG,
			Range::JPEG => AVCOL_RANGE_JPEG,
		}
	}
}
