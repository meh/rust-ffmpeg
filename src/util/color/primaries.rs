use crate::ffi::{AVColorPrimaries::*, *};
use std::ffi::CStr;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum Primaries {
	Reserved0,
	BT709,
	Unspecified,
	Reserved,
	BT470M,

	BT470BG,
	SMPTE170M,
	SMPTE240M,
	Film,
	BT2020,

	SMPTE428,
	SMPTE431,
	SMPTE432,
	#[cfg(feature = "ffmpeg_4_3")]
	EBU3213,
}

impl Primaries {
    pub fn name(&self) -> Option<&'static str> {
        if *self == Primaries::Unspecified {
            return None;
        }
        unsafe {
            let ptr = av_color_primaries_name((*self).into());
            ptr.as_ref().map(|ptr| std::str::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
        }
    }
}

impl From<AVColorPrimaries> for Primaries {
	fn from(value: AVColorPrimaries) -> Primaries {
		match value {
			AVCOL_PRI_RESERVED0 => Primaries::Reserved0,
			AVCOL_PRI_BT709 => Primaries::BT709,
			AVCOL_PRI_UNSPECIFIED => Primaries::Unspecified,
			AVCOL_PRI_RESERVED => Primaries::Reserved,
			AVCOL_PRI_BT470M => Primaries::BT470M,

			AVCOL_PRI_BT470BG => Primaries::BT470BG,
			AVCOL_PRI_SMPTE170M => Primaries::SMPTE170M,
			AVCOL_PRI_SMPTE240M => Primaries::SMPTE240M,
			AVCOL_PRI_FILM => Primaries::Film,
			AVCOL_PRI_BT2020 => Primaries::BT2020,
			AVCOL_PRI_NB => Primaries::Reserved0,

			AVCOL_PRI_SMPTE428 => Primaries::SMPTE428,
			AVCOL_PRI_SMPTE431 => Primaries::SMPTE431,
			AVCOL_PRI_SMPTE432 => Primaries::SMPTE432,
			#[cfg(feature = "ffmpeg_4_3")]
			AVCOL_PRI_EBU3213 => Primaries::EBU3213,
			#[cfg(not(feature = "ffmpeg_4_3"))]
			_ => unimplemented!(),
		}
	}
}

impl Into<AVColorPrimaries> for Primaries {
	fn into(self) -> AVColorPrimaries {
		match self {
			Primaries::Reserved0 => AVCOL_PRI_RESERVED0,
			Primaries::BT709 => AVCOL_PRI_BT709,
			Primaries::Unspecified => AVCOL_PRI_UNSPECIFIED,
			Primaries::Reserved => AVCOL_PRI_RESERVED,
			Primaries::BT470M => AVCOL_PRI_BT470M,

			Primaries::BT470BG => AVCOL_PRI_BT470BG,
			Primaries::SMPTE170M => AVCOL_PRI_SMPTE170M,
			Primaries::SMPTE240M => AVCOL_PRI_SMPTE240M,
			Primaries::Film => AVCOL_PRI_FILM,
			Primaries::BT2020 => AVCOL_PRI_BT2020,

			Primaries::SMPTE428 => AVCOL_PRI_SMPTE428,
			Primaries::SMPTE431 => AVCOL_PRI_SMPTE431,
			Primaries::SMPTE432 => AVCOL_PRI_SMPTE432,
			#[cfg(feature = "ffmpeg_4_3")]
			Primaries::EBU3213 => AVCOL_PRI_EBU3213,
		}
	}
}
