use std::{
	error,
	ffi::{CStr, CString, NulError},
	fmt::{self, Display},
	ops::Index,
	ptr, slice,
	str::{from_utf8_unchecked, FromStr},
};

use libc::{c_int, c_void};

use crate::ffi::{AVSampleFormat::*, *};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde_derive::Serialize, serde_derive::Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_", rename_all = "kebab-case"))]
pub enum Sample {
	None,

	U8(Type),
	I16(Type),
	I32(Type),
	I64(Type),
	F32(Type),
	F64(Type),
}

#[cfg_attr(feature = "serde", derive(serde_derive::Serialize, serde_derive::Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_", rename_all = "kebab-case"))]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Type {
	Packed,
	Planar,
}

impl Sample {
	#[inline]
	pub fn name(&self) -> &'static str {
		unsafe { from_utf8_unchecked(CStr::from_ptr(av_get_sample_fmt_name((*self).into())).to_bytes()) }
	}

	#[inline]
	pub fn packed(&self) -> Self {
		unsafe { Sample::from(av_get_packed_sample_fmt((*self).into())) }
	}

	#[inline]
	pub fn planar(&self) -> Self {
		unsafe { Sample::from(av_get_planar_sample_fmt((*self).into())) }
	}

	#[inline]
	pub fn is_planar(&self) -> bool {
		unsafe { av_sample_fmt_is_planar((*self).into()) == 1 }
	}

	#[inline]
	pub fn is_packed(&self) -> bool {
		!self.is_planar()
	}

	#[inline]
	pub fn bytes(&self) -> usize {
		unsafe { av_get_bytes_per_sample((*self).into()) as usize }
	}

	#[inline]
	pub fn buffer(&self, channels: u16, samples: usize, align: bool) -> Buffer {
		Buffer::new(*self, channels, samples, align)
	}
}

impl From<AVSampleFormat> for Sample {
	#[inline]
	fn from(value: AVSampleFormat) -> Self {
		match value {
			AV_SAMPLE_FMT_NONE => Sample::None,

			AV_SAMPLE_FMT_U8 => Sample::U8(Type::Packed),
			AV_SAMPLE_FMT_S16 => Sample::I16(Type::Packed),
			AV_SAMPLE_FMT_S32 => Sample::I32(Type::Packed),
			AV_SAMPLE_FMT_S64 => Sample::I64(Type::Packed),
			AV_SAMPLE_FMT_FLT => Sample::F32(Type::Packed),
			AV_SAMPLE_FMT_DBL => Sample::F64(Type::Packed),

			AV_SAMPLE_FMT_U8P => Sample::U8(Type::Planar),
			AV_SAMPLE_FMT_S16P => Sample::I16(Type::Planar),
			AV_SAMPLE_FMT_S32P => Sample::I32(Type::Planar),
			AV_SAMPLE_FMT_S64P => Sample::I64(Type::Planar),
			AV_SAMPLE_FMT_FLTP => Sample::F32(Type::Planar),
			AV_SAMPLE_FMT_DBLP => Sample::F64(Type::Planar),

			AV_SAMPLE_FMT_NB => Sample::None,
		}
	}
}

#[derive(Debug)]
pub enum ParseSampleError {
	NulError(NulError),
	UnknownFormat,
}

impl fmt::Display for ParseSampleError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			ParseSampleError::NulError(ref e) => e.fmt(f),
			ParseSampleError::UnknownFormat => write!(f, "unknown sample format"),
		}
	}
}

impl error::Error for ParseSampleError {
	fn cause(&self) -> Option<&dyn error::Error> {
		match *self {
			ParseSampleError::NulError(ref e) => Some(e),
			ParseSampleError::UnknownFormat => None,
		}
	}
}

impl From<NulError> for ParseSampleError {
	fn from(x: NulError) -> ParseSampleError {
		ParseSampleError::NulError(x)
	}
}

impl FromStr for Sample {
	type Err = ParseSampleError;

	#[inline(always)]
	fn from_str(s: &str) -> Result<Sample, ParseSampleError> {
		let value = CString::new(s)?;
		let format = unsafe { Sample::from(av_get_sample_fmt(value.as_ptr())) };

		if format == Sample::None {
			Err(ParseSampleError::UnknownFormat)
		} else {
			Ok(format)
		}
	}
}

impl Display for Sample {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let name = unsafe { av_get_sample_fmt_name((*self).into()) };
		let name = unsafe { from_utf8_unchecked(CStr::from_ptr(name).to_bytes()) };
		write!(f, "{}", name)
	}
}

impl From<&'static str> for Sample {
	#[inline]
	fn from(value: &'static str) -> Self {
		unsafe {
			let value = CString::new(value).unwrap();

			Sample::from(av_get_sample_fmt(value.as_ptr()))
		}
	}
}

impl From<Sample> for AVSampleFormat {
	#[inline]
	fn from(val: Sample) -> Self {
		match val {
			Sample::None => AV_SAMPLE_FMT_NONE,

			Sample::U8(Type::Packed) => AV_SAMPLE_FMT_U8,
			Sample::I16(Type::Packed) => AV_SAMPLE_FMT_S16,
			Sample::I32(Type::Packed) => AV_SAMPLE_FMT_S32,
			Sample::I64(Type::Packed) => AV_SAMPLE_FMT_S64,
			Sample::F32(Type::Packed) => AV_SAMPLE_FMT_FLT,
			Sample::F64(Type::Packed) => AV_SAMPLE_FMT_DBL,

			Sample::U8(Type::Planar) => AV_SAMPLE_FMT_U8P,
			Sample::I16(Type::Planar) => AV_SAMPLE_FMT_S16P,
			Sample::I32(Type::Planar) => AV_SAMPLE_FMT_S32P,
			Sample::I64(Type::Planar) => AV_SAMPLE_FMT_S64P,
			Sample::F32(Type::Planar) => AV_SAMPLE_FMT_FLTP,
			Sample::F64(Type::Planar) => AV_SAMPLE_FMT_DBLP,
		}
	}
}

pub struct Buffer {
	pub format: Sample,
	pub channels: u16,
	pub samples: usize,
	pub align: bool,

	buffer: *mut *mut u8,
	size: c_int,
}

impl Buffer {
	#[inline]
	pub fn size(format: Sample, channels: u16, samples: usize, align: bool) -> usize {
		unsafe {
			av_samples_get_buffer_size(
				ptr::null_mut(),
				i32::from(channels),
				samples as c_int,
				format.into(),
				!align as c_int,
			) as usize
		}
	}

	#[inline]
	pub fn new(format: Sample, channels: u16, samples: usize, align: bool) -> Self {
		unsafe {
			let mut buf = Buffer {
				format,
				channels,
				samples,
				align,

				buffer: ptr::null_mut(),
				size: 0,
			};

			av_samples_alloc_array_and_samples(
				&mut buf.buffer,
				&mut buf.size,
				i32::from(channels),
				samples as c_int,
				format.into(),
				!align as c_int,
			);

			buf
		}
	}
}

impl Index<usize> for Buffer {
	type Output = [u8];

	#[inline]
	fn index(&self, index: usize) -> &[u8] {
		if index >= self.samples {
			panic!("out of bounds");
		}

		unsafe { slice::from_raw_parts(*self.buffer.add(index), self.size as usize) }
	}
}

impl Clone for Buffer {
	#[inline]
	fn clone(&self) -> Self {
		let mut buf = Buffer::new(self.format, self.channels, self.samples, self.align);
		buf.clone_from(self);

		buf
	}

	#[inline]
	fn clone_from(&mut self, source: &Self) {
		unsafe {
			av_samples_copy(
				self.buffer,
				source.buffer as *const *mut u8,
				0,
				0,
				source.samples as c_int,
				i32::from(source.channels),
				source.format.into(),
			);
		}
	}
}

impl Drop for Buffer {
	#[inline]
	fn drop(&mut self) {
		unsafe {
			av_freep(self.buffer as *mut c_void);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Sample, Type};

	#[cfg(feature = "serde")]
	#[test]
	fn sample() {
		assert_eq!(
			serde_json::to_string(&Sample::U8(Type::Packed)).unwrap(),
			"{\"u8\":\"packed\"}"
		);
		assert_eq!(
			serde_json::to_string(&Sample::F64(Type::Planar)).unwrap(),
			"{\"f64\":\"planar\"}"
		);
	}

	#[test]
	fn sample_to_string() {
		assert_eq!(Sample::U8(Type::Packed).to_string(), "u8");
		assert_eq!(Sample::U8(Type::Planar).to_string(), "u8p");
		assert_eq!(Sample::F64(Type::Packed).to_string(), "dbl");
		assert_eq!(Sample::F64(Type::Planar).to_string(), "dblp");
	}
}
