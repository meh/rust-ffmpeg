use std::{marker::PhantomData, slice};

use super::Packet;
use crate::ffi::{AVPacketSideDataType::*, *};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde_derive::Serialize, serde_derive::Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_", rename_all = "kebab-case"))]
pub enum Type {
	Palette,
	NewExtraData,
	ParamChange,
	H263MbInfo,
	ReplayGain,
	DisplayMatrix,
	Stereo3d,
	AudioServiceType,
	QualityStats,
	FallbackTrack,
	CBPProperties,
	SkipSamples,
	JpDualMono,
	StringsMetadata,
	SubtitlePosition,
	MatroskaBlockAdditional,
	WebVTTIdentifier,
	WebVTTSettings,
	MetadataUpdate,
	#[cfg(feature = "ffmpeg_3_1")]
	MPEGTSStreamID,
	MasteringDisplayMetadata,
	#[cfg(feature = "ffmpeg_3_3")]
	DataSpherical,
	DataNb,

	#[cfg(feature = "ffmpeg_3_3")]
	ContentLightLevel,
	#[cfg(feature = "ffmpeg_3_4")]
	A53CC,

	#[cfg(feature = "ffmpeg_4_0")]
	EncryptionInitInfo,
	#[cfg(feature = "ffmpeg_4_0")]
	EncryptionInfo,

	#[cfg(feature = "ffmpeg_4_1")]
	AFD,
}

impl From<AVPacketSideDataType> for Type {
	fn from(value: AVPacketSideDataType) -> Self {
		#[allow(unreachable_patterns)]
		match value {
			AV_PKT_DATA_PALETTE => Type::Palette,
			AV_PKT_DATA_NEW_EXTRADATA => Type::NewExtraData,
			AV_PKT_DATA_PARAM_CHANGE => Type::ParamChange,
			AV_PKT_DATA_H263_MB_INFO => Type::H263MbInfo,
			AV_PKT_DATA_REPLAYGAIN => Type::ReplayGain,
			AV_PKT_DATA_DISPLAYMATRIX => Type::DisplayMatrix,
			AV_PKT_DATA_STEREO3D => Type::Stereo3d,
			AV_PKT_DATA_AUDIO_SERVICE_TYPE => Type::AudioServiceType,
			AV_PKT_DATA_QUALITY_STATS => Type::QualityStats,
			AV_PKT_DATA_FALLBACK_TRACK => Type::FallbackTrack,
			AV_PKT_DATA_CPB_PROPERTIES => Type::CBPProperties,
			AV_PKT_DATA_SKIP_SAMPLES => Type::SkipSamples,
			AV_PKT_DATA_JP_DUALMONO => Type::JpDualMono,
			AV_PKT_DATA_STRINGS_METADATA => Type::StringsMetadata,
			AV_PKT_DATA_SUBTITLE_POSITION => Type::SubtitlePosition,
			AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL => Type::MatroskaBlockAdditional,
			AV_PKT_DATA_WEBVTT_IDENTIFIER => Type::WebVTTIdentifier,
			AV_PKT_DATA_WEBVTT_SETTINGS => Type::WebVTTSettings,
			AV_PKT_DATA_METADATA_UPDATE => Type::MetadataUpdate,
			#[cfg(feature = "ffmpeg_3_1")]
			AV_PKT_DATA_MPEGTS_STREAM_ID => Type::MPEGTSStreamID,
			AV_PKT_DATA_MASTERING_DISPLAY_METADATA => Type::MasteringDisplayMetadata,
			#[cfg(feature = "ffmpeg_3_3")]
			AV_PKT_DATA_SPHERICAL => Type::DataSpherical,
			AV_PKT_DATA_NB => Type::DataNb,

			#[cfg(feature = "ffmpeg_3_3")]
			AV_PKT_DATA_CONTENT_LIGHT_LEVEL => Type::ContentLightLevel,
			#[cfg(feature = "ffmpeg_3_4")]
			AV_PKT_DATA_A53_CC => Type::A53CC,
			#[cfg(feature = "ffmpeg_4_0")]
			AV_PKT_DATA_ENCRYPTION_INIT_INFO => Type::EncryptionInitInfo,
			#[cfg(feature = "ffmpeg_4_0")]
			AV_PKT_DATA_ENCRYPTION_INFO => Type::EncryptionInfo,
			#[cfg(feature = "ffmpeg_4_1")]
			AV_PKT_DATA_AFD => Type::AFD,

			_ => unimplemented!(),
		}
	}
}

impl Into<AVPacketSideDataType> for Type {
	fn into(self) -> AVPacketSideDataType {
		match self {
			Type::Palette => AV_PKT_DATA_PALETTE,
			Type::NewExtraData => AV_PKT_DATA_NEW_EXTRADATA,
			Type::ParamChange => AV_PKT_DATA_PARAM_CHANGE,
			Type::H263MbInfo => AV_PKT_DATA_H263_MB_INFO,
			Type::ReplayGain => AV_PKT_DATA_REPLAYGAIN,
			Type::DisplayMatrix => AV_PKT_DATA_DISPLAYMATRIX,
			Type::Stereo3d => AV_PKT_DATA_STEREO3D,
			Type::AudioServiceType => AV_PKT_DATA_AUDIO_SERVICE_TYPE,
			Type::QualityStats => AV_PKT_DATA_QUALITY_STATS,
			Type::FallbackTrack => AV_PKT_DATA_FALLBACK_TRACK,
			Type::CBPProperties => AV_PKT_DATA_CPB_PROPERTIES,
			Type::SkipSamples => AV_PKT_DATA_SKIP_SAMPLES,
			Type::JpDualMono => AV_PKT_DATA_JP_DUALMONO,
			Type::StringsMetadata => AV_PKT_DATA_STRINGS_METADATA,
			Type::SubtitlePosition => AV_PKT_DATA_SUBTITLE_POSITION,
			Type::MatroskaBlockAdditional => AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL,
			Type::WebVTTIdentifier => AV_PKT_DATA_WEBVTT_IDENTIFIER,
			Type::WebVTTSettings => AV_PKT_DATA_WEBVTT_SETTINGS,
			Type::MetadataUpdate => AV_PKT_DATA_METADATA_UPDATE,
			#[cfg(feature = "ffmpeg_3_1")]
			Type::MPEGTSStreamID => AV_PKT_DATA_MPEGTS_STREAM_ID,
			Type::MasteringDisplayMetadata => AV_PKT_DATA_MASTERING_DISPLAY_METADATA,
			#[cfg(feature = "ffmpeg_3_3")]
			Type::DataSpherical => AV_PKT_DATA_SPHERICAL,
			Type::DataNb => AV_PKT_DATA_NB,

			#[cfg(feature = "ffmpeg_3_3")]
			Type::ContentLightLevel => AV_PKT_DATA_CONTENT_LIGHT_LEVEL,
			#[cfg(feature = "ffmpeg_3_4")]
			Type::A53CC => AV_PKT_DATA_A53_CC,
			#[cfg(feature = "ffmpeg_4_0")]
			Type::EncryptionInitInfo => AV_PKT_DATA_ENCRYPTION_INIT_INFO,
			#[cfg(feature = "ffmpeg_4_0")]
			Type::EncryptionInfo => AV_PKT_DATA_ENCRYPTION_INFO,
			#[cfg(feature = "ffmpeg_4_1")]
			Type::AFD => AV_PKT_DATA_AFD,
		}
	}
}

pub struct SideData<'a> {
	ptr: *mut AVPacketSideData,

	_marker: PhantomData<&'a Packet>,
}

impl<'a> SideData<'a> {
	pub unsafe fn wrap(ptr: *mut AVPacketSideData) -> Self {
		SideData {
			ptr,
			_marker: PhantomData,
		}
	}

	pub unsafe fn as_ptr(&self) -> *const AVPacketSideData {
		self.ptr as *const _
	}
}

impl<'a> SideData<'a> {
	pub fn kind(&self) -> Type {
		unsafe { Type::from((*self.as_ptr()).type_) }
	}

	pub fn data(&self) -> &[u8] {
		unsafe { slice::from_raw_parts((*self.as_ptr()).data, (*self.as_ptr()).size as usize) }
	}
}
