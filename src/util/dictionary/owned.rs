use std::{fmt, iter::FromIterator, ptr};

use super::{immutable, mutable, Iter};
use crate::ffi::*;

pub struct Owned {
	ptr: *mut AVDictionary,
}

unsafe impl Send for Owned {}

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

	pub unsafe fn as_ptr(&self) -> *const AVDictionary {
		self.ptr
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVDictionary {
		self.ptr
	}
}

impl Owned {
	pub fn new() -> Self {
		Owned { ptr: ptr::null_mut() }
	}

	pub fn as_ref(&self) -> immutable::Ref {
		unsafe { immutable::Ref::wrap(self.ptr) }
	}

	pub fn len(&self) -> usize {
		self.as_ref().len()
	}

	pub fn iter(&self) -> Iter<'_> {
		unsafe { Iter::new(self.as_ptr()) }
	}

	pub fn set(&mut self, key: &str, value: &str) -> &mut Self {
		unsafe {
			let mut mutable = mutable::Ref::wrap(self.ptr);
			mutable.set(key, value);
			self.ptr = mutable.as_mut_ptr();
		}

		self
	}
}

impl<'a> FromIterator<(&'a str, &'a str)> for Owned {
	fn from_iter<T: IntoIterator<Item = (&'a str, &'a str)>>(iterator: T) -> Self {
		let mut result = Owned::new();

		for (key, value) in iterator {
			result.set(key, value);
		}

		result
	}
}

impl<'a> FromIterator<&'a (&'a str, &'a str)> for Owned {
	fn from_iter<T: IntoIterator<Item = &'a (&'a str, &'a str)>>(iterator: T) -> Self {
		let mut result = Owned::new();

		for &(key, value) in iterator {
			result.set(key, value);
		}

		result
	}
}

impl FromIterator<(String, String)> for Owned {
	fn from_iter<T: IntoIterator<Item = (String, String)>>(iterator: T) -> Self {
		let mut result = Owned::new();

		for (key, value) in iterator {
			result.set(&key, &value);
		}

		result
	}
}

impl<'a> FromIterator<&'a (String, String)> for Owned {
	fn from_iter<T: IntoIterator<Item = &'a (String, String)>>(iterator: T) -> Self {
		let mut result = Owned::new();

		for &(ref key, ref value) in iterator {
			result.set(key, value);
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

#[cfg(feature = "serde")]
mod serde {
	use std::fmt;

	use ::serde_::{
		de::{Deserialize, Deserializer, MapAccess, Visitor},
		ser::{Serialize, SerializeMap, Serializer},
	};

	impl Serialize for super::Owned {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let mut map = serializer.serialize_map(Some(self.len()))?;
			for (k, v) in self.iter() {
				map.serialize_entry(k, v)?;
			}
			map.end()
		}
	}

	struct DictionaryVisitor;

	impl<'de> Visitor<'de> for DictionaryVisitor {
		type Value = super::Owned;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("an ffmpeg dictionary")
		}

		fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
		where
			M: MapAccess<'de>,
		{
			let mut map = super::Owned::new();

			while let Some((key, value)) = access.next_entry()? {
				map.set(key, value);
			}

			Ok(map)
		}
	}

	impl<'de> Deserialize<'de> for super::Owned {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			deserializer.deserialize_map(DictionaryVisitor)
		}
	}
}
