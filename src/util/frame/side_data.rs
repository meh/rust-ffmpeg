use std::{ffi::CStr, marker::PhantomData, slice, str::from_utf8_unchecked};

use super::Frame;
use crate::{
	ffi::{AVFrameSideDataType::*, *},
	DictionaryRef,
};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Type {
	PanScan,
	A53CC,
	Stereo3D,
	MatrixEncoding,
	DownMixInfo,
	ReplayGain,
	DisplayMatrix,
	AFD,
	MotionVectors,
	SkipSamples,
	AudioServiceType,
	MasteringDisplayMetadata,
	GOPTimecode,
	#[cfg(feature = "ffmpeg_3_3")]
	Spherical,

	#[cfg(feature = "ffmpeg_3_3")]
	ContentLightLevel,
	#[cfg(feature = "ffmpeg_3_4")]
	IccProfile,

	#[cfg(all(feature = "ffmpeg_4_0", not(feature = "ffmpeg_5_0")))]
	QpTableProperties,

	#[cfg(all(feature = "ffmpeg_4_0", not(feature = "ffmpeg_5_0")))]
	QpTableData,

	#[cfg(feature = "ffmpeg_4_1")]
	S12MTimecode,
}

impl Type {
	#[inline]
	pub fn name(&self) -> &'static str {
		unsafe { from_utf8_unchecked(CStr::from_ptr(av_frame_side_data_name((*self).into())).to_bytes()) }
	}
}

impl From<AVFrameSideDataType> for Type {
	#[inline(always)]
	fn from(value: AVFrameSideDataType) -> Self {
		#[allow(unreachable_patterns)]
		match value {
			AV_FRAME_DATA_PANSCAN => Type::PanScan,
			AV_FRAME_DATA_A53_CC => Type::A53CC,
			AV_FRAME_DATA_STEREO3D => Type::Stereo3D,
			AV_FRAME_DATA_MATRIXENCODING => Type::MatrixEncoding,
			AV_FRAME_DATA_DOWNMIX_INFO => Type::DownMixInfo,
			AV_FRAME_DATA_REPLAYGAIN => Type::ReplayGain,
			AV_FRAME_DATA_DISPLAYMATRIX => Type::DisplayMatrix,
			AV_FRAME_DATA_AFD => Type::AFD,
			AV_FRAME_DATA_MOTION_VECTORS => Type::MotionVectors,
			AV_FRAME_DATA_SKIP_SAMPLES => Type::SkipSamples,
			AV_FRAME_DATA_AUDIO_SERVICE_TYPE => Type::AudioServiceType,
			AV_FRAME_DATA_MASTERING_DISPLAY_METADATA => Type::MasteringDisplayMetadata,
			AV_FRAME_DATA_GOP_TIMECODE => Type::GOPTimecode,
			#[cfg(feature = "ffmpeg_3_3")]
			AV_FRAME_DATA_SPHERICAL => Type::Spherical,

			#[cfg(feature = "ffmpeg_3_3")]
			AV_FRAME_DATA_CONTENT_LIGHT_LEVEL => Type::ContentLightLevel,
			#[cfg(feature = "ffmpeg_3_4")]
			AV_FRAME_DATA_ICC_PROFILE => Type::IccProfile,
			// TODO It's undef #if in frame.h
			#[cfg(all(feature = "ffmpeg_4_0", not(feature = "ffmpeg_5_0")))]
			AV_FRAME_DATA_QP_TABLE_PROPERTIES => Type::QpTableProperties,
			#[cfg(all(feature = "ffmpeg_4_0", not(feature = "ffmpeg_5_0")))]
			AV_FRAME_DATA_QP_TABLE_DATA => Type::QpTableData,
			#[cfg(feature = "ffmpeg_4_1")]
			AV_FRAME_DATA_S12M_TIMECODE => Type::S12MTimecode,

			_ => unimplemented!(),
		}
	}
}

impl Into<AVFrameSideDataType> for Type {
	#[inline(always)]
	fn into(self) -> AVFrameSideDataType {
		match self {
			Type::PanScan => AV_FRAME_DATA_PANSCAN,
			Type::A53CC => AV_FRAME_DATA_A53_CC,
			Type::Stereo3D => AV_FRAME_DATA_STEREO3D,
			Type::MatrixEncoding => AV_FRAME_DATA_MATRIXENCODING,
			Type::DownMixInfo => AV_FRAME_DATA_DOWNMIX_INFO,
			Type::ReplayGain => AV_FRAME_DATA_REPLAYGAIN,
			Type::DisplayMatrix => AV_FRAME_DATA_DISPLAYMATRIX,
			Type::AFD => AV_FRAME_DATA_AFD,
			Type::MotionVectors => AV_FRAME_DATA_MOTION_VECTORS,
			Type::SkipSamples => AV_FRAME_DATA_SKIP_SAMPLES,
			Type::AudioServiceType => AV_FRAME_DATA_AUDIO_SERVICE_TYPE,
			Type::MasteringDisplayMetadata => AV_FRAME_DATA_MASTERING_DISPLAY_METADATA,
			Type::GOPTimecode => AV_FRAME_DATA_GOP_TIMECODE,
			#[cfg(feature = "ffmpeg_3_3")]
			Type::Spherical => AV_FRAME_DATA_SPHERICAL,

			#[cfg(feature = "ffmpeg_3_3")]
			Type::ContentLightLevel => AV_FRAME_DATA_CONTENT_LIGHT_LEVEL,
			#[cfg(feature = "ffmpeg_3_4")]
			Type::IccProfile => AV_FRAME_DATA_ICC_PROFILE,

			#[cfg(all(feature = "ffmpeg_4_0", not(feature = "ffmpeg_5_0")))]
			Type::QpTableProperties => AV_FRAME_DATA_QP_TABLE_PROPERTIES,
			#[cfg(all(feature = "ffmpeg_4_0", not(feature = "ffmpeg_5_0")))]
			Type::QpTableData => AV_FRAME_DATA_QP_TABLE_DATA,

			#[cfg(feature = "ffmpeg_4_1")]
			Type::S12MTimecode => AV_FRAME_DATA_S12M_TIMECODE,
		}
	}
}

pub struct SideData<'a> {
	ptr: *mut AVFrameSideData,

	_marker: PhantomData<&'a Frame>,
}

impl<'a> SideData<'a> {
	#[inline(always)]
	pub unsafe fn wrap(ptr: *mut AVFrameSideData) -> Self {
		SideData {
			ptr,
			_marker: PhantomData,
		}
	}

	#[inline(always)]
	pub unsafe fn as_ptr(&self) -> *const AVFrameSideData {
		self.ptr as *const _
	}

	#[inline(always)]
	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFrameSideData {
		self.ptr
	}
}

impl<'a> SideData<'a> {
	#[inline]
	pub fn kind(&self) -> Type {
		unsafe { Type::from((*self.as_ptr()).type_) }
	}

	#[inline]
	pub fn data(&self) -> &[u8] {
		unsafe { slice::from_raw_parts((*self.as_ptr()).data, (*self.as_ptr()).size as usize) }
	}

	#[inline]
	pub fn metadata(&self) -> DictionaryRef<'_> {
		unsafe { DictionaryRef::wrap((*self.as_ptr()).metadata) }
	}
}
