use crate::ffi::{AVColorRange::*, *};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum Range {
	Unspecified,
	MPEG,
	JPEG,
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
