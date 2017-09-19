use std::ops::Deref;
use std::mem;

use ffi::*;
use ::{Rational, Dictionary, DictionaryMut};
use super::Chapter;
use format::context::common::Context;

// WARNING: index refers to the offset in the chapters array (starting from 0)
// it is not necessarly equal to the id (which may start at 1)
pub struct ChapterMut<'a> {
	context: &'a mut Context,
	index:   usize,

	immutable: Chapter<'a>,
}

impl<'a> ChapterMut<'a> {
	pub unsafe fn wrap(context: &mut Context, index: usize) -> ChapterMut {
		ChapterMut {
			context: mem::transmute_copy(&context),
			index:   index,

			immutable: Chapter::wrap(mem::transmute_copy(&context), index)
		}
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVChapter {
		*(*self.context.as_mut_ptr()).chapters.offset(self.index as isize)
	}
}

impl<'a> ChapterMut<'a> {
	pub fn set_id(&mut self, value: i32) {
		unsafe {
			(*self.as_mut_ptr()).id = value;
		}
	}

	pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
		unsafe {
			(*self.as_mut_ptr()).time_base = value.into().into();
		}
	}

	pub fn set_start(&mut self, value: i64) {
		unsafe {
			(*self.as_mut_ptr()).start = value;
		}
	}

	pub fn set_end(&mut self, value: i64) {
		unsafe {
			(*self.as_mut_ptr()).end = value;
		}
	}

	pub fn set_metadata(&mut self, key: &str, value: &str) {
        // dictionary.set() allocates the AVDictionary the first time a key/value is inserted
        // so we want to update the metadata dictionary afterwards
		unsafe {
	        let mut dictionary = Dictionary::own(self.metadata().as_mut_ptr());
            dictionary.set(key, value);
			(*self.as_mut_ptr()).metadata = dictionary.disown();
		}
	}

	pub fn metadata(&mut self) -> DictionaryMut {
		unsafe {
			DictionaryMut::wrap((*self.as_mut_ptr()).metadata)
		}
	}
}

impl<'a> Deref for ChapterMut<'a> {
	type Target = Chapter<'a>;

	fn deref(&self) -> &Self::Target {
		&self.immutable
	}
}
