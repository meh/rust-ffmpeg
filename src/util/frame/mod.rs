pub mod side_data;
pub use self::side_data::SideData;

pub mod video;
pub use self::video::Video;

pub mod audio;
pub use self::audio::Audio;

pub mod flag;

pub use self::flag::Flags;
use crate::{ffi::*, Dictionary, DictionaryRef, Error, Rational};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Packet {
	pub duration: i64,
	pub position: i64,
	pub size: usize,

	pub pts: i64,
	pub dts: i64,
}

#[derive(PartialEq, Eq)]
pub struct Frame {
	ptr: *mut AVFrame,

	_own: bool,
}

unsafe impl Send for Frame {}
unsafe impl Sync for Frame {}

impl Frame {
	#[inline(always)]
	pub unsafe fn wrap(ptr: *mut AVFrame) -> Self {
		Frame { ptr, _own: false }
	}

	#[inline(always)]
	pub fn empty() -> Self {
		unsafe {
			Frame {
				ptr: av_frame_alloc(),
				_own: true,
			}
		}
	}

	#[inline(always)]
	pub fn as_ptr(&self) -> *const AVFrame {
		self.ptr as *const _
	}

	#[inline(always)]
	pub fn as_mut_ptr(&mut self) -> *mut AVFrame {
		self.ptr
	}

	#[inline(always)]
	pub fn is_empty(&self) -> bool {
		unsafe { (*self.as_ptr()).data[0].is_null() }
	}
}

impl Frame {
	#[inline]
	pub fn is_key(&self) -> bool {
		unsafe { (*self.as_ptr()).key_frame == 1 }
	}

	#[inline]
	pub fn is_corrupt(&self) -> bool {
		self.flags().contains(Flags::CORRUPT)
	}

	#[inline]
	pub fn packet(&self) -> Packet {
		#[cfg(feature = "ffmpeg_3_2")]
		unsafe {
			Packet {
				duration: (*self.as_ptr()).pkt_duration as i64,
				position: (*self.as_ptr()).pkt_pos as i64,
				size: (*self.as_ptr()).pkt_size as usize,

				pts: (*self.as_ptr()).pts,
				dts: (*self.as_ptr()).pkt_dts,
			}
		}

		#[cfg(not(feature = "ffmpeg_3_2"))]
		unsafe {
			Packet {
				duration: (*self.as_ptr()).pkt_duration as i64,
				position: (*self.as_ptr()).pkt_pos as i64,
				size: (*self.as_ptr()).pkt_size as usize,

				pts: (*self.as_ptr()).pkt_pts,
				dts: (*self.as_ptr()).pkt_dts,
			}
		}
	}

	#[inline]
	pub fn time_base(&self) -> Option<Rational> {
		unsafe { Rational::from((*self.as_ptr()).time_base).non_zero() }
	}

	#[inline]
	pub fn set_time_base(&mut self, time_base: Option<impl Into<Rational>>) {
		unsafe {
			(*self.as_mut_ptr()).time_base = time_base.map(Into::into).unwrap_or(Rational::ZERO).into();
		}
	}

	#[inline]
	pub fn pts(&self) -> Option<i64> {
		unsafe {
			match (*self.as_ptr()).pts {
				AV_NOPTS_VALUE => None,
				pts => Some(pts as i64),
			}
		}
	}

	#[inline]
	pub fn set_pts(&mut self, value: Option<i64>) {
		unsafe {
			(*self.as_mut_ptr()).pts = value.unwrap_or(AV_NOPTS_VALUE);
		}
	}

	#[inline]
	pub fn timestamp(&self) -> Option<i64> {
		unsafe {
			match (*self.as_ptr()).best_effort_timestamp {
				AV_NOPTS_VALUE => None,
				t => Some(t as i64),
			}
		}
	}

	#[inline]
	#[cfg(feature = "ffmpeg_6_0")]
	pub fn duration(&self) -> Option<i64> {
		unsafe {
			match (*self.as_ptr()).duration {
				0 => None,
				d => Some(d),
			}
		}
	}

	#[inline]
	#[cfg(feature = "ffmpeg_6_0")]
	pub fn set_duration(&mut self, value: Option<i64>) {
		unsafe {
			(*self.as_mut_ptr()).duration = value.unwrap_or(0);
		}
	}

	#[inline]
	pub fn quality(&self) -> usize {
		unsafe { (*self.as_ptr()).quality as usize }
	}

	#[inline]
	pub fn flags(&self) -> Flags {
		unsafe { Flags::from_bits_truncate((*self.as_ptr()).flags) }
	}

	#[inline]
	pub fn metadata(&self) -> DictionaryRef<'_> {
		unsafe { DictionaryRef::wrap((*self.as_ptr()).metadata) }
	}

	#[inline]
	pub fn set_metadata(&mut self, value: Dictionary) {
		unsafe {
			(*self.as_mut_ptr()).metadata = value.disown();
		}
	}

	#[inline]
	pub fn side_data(&self, kind: side_data::Type) -> Option<SideData<'_>> {
		unsafe {
			let ptr = av_frame_get_side_data(self.as_ptr(), kind.into());

			if ptr.is_null() {
				None
			} else {
				Some(SideData::wrap(ptr))
			}
		}
	}

	#[inline]
	pub fn new_side_data(&mut self, kind: side_data::Type, size: usize) -> Option<SideData<'_>> {
		unsafe {
			#[cfg(feature = "ffmpeg_5_0")]
			let size = size as libc::size_t;
			#[cfg(not(feature = "ffmpeg_5_0"))]
			let size = size as libc::c_int;

			let ptr = av_frame_new_side_data(self.as_mut_ptr(), kind.into(), size);

			if ptr.is_null() {
				None
			} else {
				Some(SideData::wrap(ptr))
			}
		}
	}

	#[inline]
	pub fn remove_side_data(&mut self, kind: side_data::Type) {
		unsafe {
			av_frame_remove_side_data(self.as_mut_ptr(), kind.into());
		}
	}

	#[inline]
	pub fn is_writable(&mut self) -> Result<(), Error> {
		unsafe {
			match av_frame_is_writable(self.as_mut_ptr()) {
				0 => Ok(()),
				e => Err(Error::from(e)),
			}
		}
	}

	#[inline]
	pub fn make_writable(&mut self) -> Result<(), Error> {
		unsafe {
			match av_frame_make_writable(self.as_mut_ptr()) {
				0 => Ok(()),
				e => Err(Error::from(e)),
			}
		}
	}

	#[inline]
	pub fn try_clone(&self) -> Result<Self, Error> {
		unsafe {
			// This doesn't use av_frame_clone, because it masks
			// any av_frame_ref errors by returning a null pointer.

			let mut frame = Frame::empty();

			match av_frame_ref(frame.as_mut_ptr(), self.as_ptr()) {
				0 => Ok(frame),
				e => Err(Error::from(e)),
			}
		}
	}
}

impl Clone for Frame {
	#[inline]
	fn clone(&self) -> Self {
		unsafe {
			let mut frame = Frame::wrap(av_frame_clone(self.as_ptr()));
			frame._own = true;
			frame
		}
	}

	#[inline]
	fn clone_from(&mut self, source: &Self) {
		unsafe {
			av_frame_copy(self.as_mut_ptr(), source.as_ptr());
			av_frame_copy_props(self.as_mut_ptr(), source.as_ptr());
		}
	}
}

impl Drop for Frame {
	#[inline]
	fn drop(&mut self) {
		unsafe {
			if self._own {
				av_frame_free(&mut self.as_mut_ptr());
			}
		}
	}
}
