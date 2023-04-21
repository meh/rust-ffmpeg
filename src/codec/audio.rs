use std::ops::Deref;

use super::codec::Codec;
use crate::{ffi::*, format, ChannelLayout};

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Audio {
	codec: Codec,
}

impl Audio {
	pub unsafe fn new(codec: Codec) -> Audio {
		Audio { codec }
	}
}

impl Audio {
	pub fn sample_rates(&self) -> Option<RateIter> {
		unsafe {
			(!(*self.as_ptr()).supported_samplerates.is_null()).then_some(RateIter {
				inner: self,
				next_idx: 0,
			})
		}
	}

	pub fn formats(&self) -> Option<FormatIter> {
		unsafe {
			(!(*self.codec.as_ptr()).sample_fmts.is_null()).then_some(FormatIter {
				inner: self,
				next_idx: 0,
			})
		}
	}

	pub fn channel_layouts(&self) -> Option<ChannelLayoutIter> {
		unsafe {
			(!(*self.codec.as_ptr()).ch_layouts.is_null()).then_some(ChannelLayoutIter {
				inner: self,
				next_idx: 0,
			})
		}
	}
}

impl Deref for Audio {
	type Target = Codec;

	fn deref(&self) -> &Self::Target {
		&self.codec
	}
}

pub struct RateIter<'a> {
	inner: &'a Audio,
	next_idx: isize,
}

impl Iterator for RateIter<'_> {
	type Item = i32;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		unsafe {
			let all = (*self.inner.codec.as_ptr()).supported_samplerates;
			let elem = *all.offset(self.next_idx);

			if elem == 0 {
				None
			}
			else {
				self.next_idx += 1;
				Some(elem)
			}
		}
	}
}

pub struct FormatIter<'a> {
	inner: &'a Audio,
	next_idx: isize,
}

impl Iterator for FormatIter<'_> {
	type Item = format::Sample;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		unsafe {
			let all = (*self.inner.codec.as_ptr()).sample_fmts;
			let elem = *all.offset(self.next_idx);

			if elem == AVSampleFormat::AV_SAMPLE_FMT_NONE {
				None
			}
			else {
				self.next_idx += 1;
				Some(elem.into())
			}
		}
	}
}

pub struct ChannelLayoutIter<'a> {
	inner: &'a Audio,
	next_idx: isize,
}

impl ChannelLayoutIter<'_> {
	pub fn best(self, max: i32) -> ChannelLayout {
		self.fold(crate::channel_layout::ChannelLayout::MONO, |acc, cur| {
			if cur.channels() > acc.channels() && cur.channels() <= max {
				cur
			}
			else {
				acc
			}
		})
	}
}

impl Iterator for ChannelLayoutIter<'_> {
	type Item = ChannelLayout;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		unsafe {
			let all = (*self.inner.codec.as_ptr()).ch_layouts;
			let elem = ChannelLayout::from(*all.offset(self.next_idx));

			// ... terminated with a zeroed layout.
			if elem.is_zeroed() {
				None
			}
			else {
				self.next_idx += 1;
				Some(elem)
			}
		}
	}
}
