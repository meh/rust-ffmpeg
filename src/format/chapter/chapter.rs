use crate::{ffi::*, format::context::common::Context, DictionaryRef, Rational};

#[cfg(feature = "ffmpeg_5_0")]
pub type ChapterId = i64;

#[cfg(not(feature = "ffmpeg_5_0"))]
pub type ChapterId = i32;

// WARNING: index refers to the offset in the chapters array (starting from 0)
// it is not necessarly equal to the id (which may start at 1)
pub struct Chapter<'a> {
	context: &'a Context,
	index: usize,
}

impl<'a> Chapter<'a> {
	pub unsafe fn wrap(context: &Context, index: usize) -> Chapter<'_> {
		Chapter { context, index }
	}

	pub unsafe fn as_ptr(&self) -> *const AVChapter {
		*(*self.context.as_ptr()).chapters.add(self.index)
	}
}

impl<'a> Chapter<'a> {
	pub fn index(&self) -> usize {
		self.index
	}

	pub fn id(&self) -> ChapterId {
		unsafe { (*self.as_ptr()).id }
	}

	pub fn time_base(&self) -> Option<Rational> {
		unsafe { Rational::from((*self.as_ptr()).time_base).non_zero() }
	}

	pub fn start(&self) -> i64 {
		unsafe { (*self.as_ptr()).start }
	}

	pub fn end(&self) -> i64 {
		unsafe { (*self.as_ptr()).end }
	}

	pub fn metadata(&self) -> DictionaryRef<'_> {
		unsafe { DictionaryRef::wrap((*self.as_ptr()).metadata) }
	}
}

impl<'a> PartialEq for Chapter<'a> {
	fn eq(&self, other: &Self) -> bool {
		unsafe { self.as_ptr() == other.as_ptr() }
	}
}
