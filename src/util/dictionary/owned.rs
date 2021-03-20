use std::{fmt, iter::FromIterator, ptr};

use super::{immutable, mutable};
use crate::ffi::*;

pub struct Owned {
	ptr: *mut AVDictionary,
}

impl Default for Owned {
	fn default() -> Self {
		Self::new()
	}
}

impl Owned {
	pub unsafe fn own(ptr: *mut AVDictionary) -> Self {
		Owned { ptr }
	}

	pub unsafe fn disown(mut self) -> *mut AVDictionary {
		let result = self.ptr;
		self.ptr = ptr::null_mut();
		result
	}

	pub fn as_ptr(&self) -> *const AVDictionary {
		self.ptr
	}

	pub fn as_mut_ptr(&mut self) -> *mut AVDictionary {
		self.ptr
	}
}

impl Owned {
	pub fn new() -> Self {
		Owned {
			ptr: ptr::null_mut(),
		}
	}

	pub fn as_ref(&self) -> immutable::Ref {
		unsafe { immutable::Ref::wrap(self.ptr) }
	}

	pub fn as_mut(&self) -> mutable::Ref {
		unsafe { mutable::Ref::wrap(self.ptr) }
	}
}

impl<'a> FromIterator<(&'a str, &'a str)> for Owned {
	fn from_iter<T: IntoIterator<Item = (&'a str, &'a str)>>(iterator: T) -> Self {
		let result = Owned::new();

		for (key, value) in iterator {
			result.as_mut().set(key, value);
		}

		result
	}
}

impl<'a> FromIterator<&'a (&'a str, &'a str)> for Owned {
	fn from_iter<T: IntoIterator<Item = &'a (&'a str, &'a str)>>(iterator: T) -> Self {
		let result = Owned::new();

		for &(key, value) in iterator {
			result.as_mut().set(key, value);
		}

		result
	}
}

impl FromIterator<(String, String)> for Owned {
	fn from_iter<T: IntoIterator<Item = (String, String)>>(iterator: T) -> Self {
		let result = Owned::new();

		for (key, value) in iterator {
			result.as_mut().set(&key, &value);
		}

		result
	}
}

impl<'a> FromIterator<&'a (String, String)> for Owned {
	fn from_iter<T: IntoIterator<Item = &'a (String, String)>>(iterator: T) -> Self {
		let result = Owned::new();

		for &(ref key, ref value) in iterator {
			result.as_mut().set(key, value);
		}

		result
	}
}

impl Clone for Owned {
	fn clone(&self) -> Self {
		let mut dictionary = Owned::new();
		dictionary.clone_from(self);

		dictionary
	}

	fn clone_from(&mut self, source: &Self) {
		unsafe {
			av_dict_copy(&mut self.ptr, source.as_ptr(), 0);
		}
	}
}

impl Drop for Owned {
	fn drop(&mut self) {
		unsafe {
			if !self.ptr.is_null() {
				av_dict_free(&mut self.ptr);
			}
		}
	}
}

impl fmt::Debug for Owned {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		unsafe { mutable::Ref::wrap(self.ptr) }.fmt(fmt)
	}
}
