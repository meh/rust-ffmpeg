use std::marker::PhantomData;
use std::ptr;
use std::ffi::{CStr, CString};
use std::str::from_utf8_unchecked;

use ffi::*;

pub struct Dictionary<'a> {
	ptr: *mut AVDictionary,

	_own:    bool,
	_marker: PhantomData<&'a ()>,
}

impl<'a> Dictionary<'a> {
	pub unsafe fn wrap(ptr: *mut AVDictionary) -> Self {
		Dictionary { ptr: ptr, _own: false, _marker: PhantomData }
	}

	pub unsafe fn own(ptr: *mut AVDictionary) -> Self {
		Dictionary { ptr: ptr, _own: true, _marker: PhantomData }
	}

	pub unsafe fn as_ptr(&self) -> *const AVDictionary {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVDictionary {
		self.ptr
	}

	pub unsafe fn take(mut self) -> *mut AVDictionary {
		self._own = false;
		self.ptr
	}
}

impl<'a> Dictionary<'a> {
	pub fn new() -> Self {
		Dictionary { ptr: ptr::null_mut(), _own: true, _marker: PhantomData }
	}

	pub fn set(&mut self, key: &str, value: &str) {
		let key_cstr = CString::new(key).unwrap();
		let value_cstr = CString::new(value).unwrap();
		unsafe {
			let mut ptr = self.as_mut_ptr();
			if av_dict_set(&mut ptr as *mut _, key_cstr.as_ptr(), value_cstr.as_ptr(), 0) >= 0 {
				self.ptr = ptr;
			}
		}
	}

	pub fn get(&self, key: &str) -> Option<&str> {
		let key_cstr = CString::new(key).unwrap();
		unsafe {
			let entry = av_dict_get(self.as_ptr(), key_cstr.as_ptr(), ptr::null_mut(), 0);

			if !entry.is_null() {
				let value = from_utf8_unchecked(CStr::from_ptr((*entry).value).to_bytes());
				Some(value)
			}
			else {
				None
			}
		}
	}

	pub fn iter(&self) -> DictionaryIter {
		unsafe {
			DictionaryIter::new(self.as_ptr())
		}
	}
}

impl<'a> Drop for Dictionary<'a> {
	fn drop(&mut self) {
		unsafe {
			if self._own && self.as_ptr() != ptr::null() {
				av_dict_free(&mut self.as_mut_ptr());
			}
		}
	}
}

impl<'a> Clone for Dictionary<'a> {
	fn clone(&self) -> Self {
		let mut dictionary = Dictionary::new();
		dictionary.clone_from(self);

		dictionary
	}

	fn clone_from(&mut self, source: &Self) {
		unsafe {
			let mut ptr = self.as_mut_ptr();
			av_dict_copy(&mut ptr as *mut _, source.as_ptr(), AV_DICT_DONT_OVERWRITE);
			self.ptr = ptr;
		}
	}
}

pub struct DictionaryIter<'a> {
	ptr: *const AVDictionary,
	cur: *mut AVDictionaryEntry,

	_marker: PhantomData<&'a Dictionary<'a>>,
}

impl<'a> DictionaryIter<'a> {
	pub fn new(dictionary: *const AVDictionary) -> Self {
		DictionaryIter {
			ptr: dictionary,
			cur: ptr::null_mut(),
			
			_marker: PhantomData
		}
	}
}

impl<'a> Iterator for DictionaryIter<'a> {
	type Item = (&'a str, &'a str);

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		unsafe {
			let empty = CString::new("").unwrap();
			let entry = av_dict_get(self.ptr, empty.as_ptr(), self.cur, AV_DICT_IGNORE_SUFFIX);

			if !entry.is_null() {
				let key = from_utf8_unchecked(CStr::from_ptr((*entry).key).to_bytes());
				let val = from_utf8_unchecked(CStr::from_ptr((*entry).value).to_bytes());

				self.cur = entry;

				Some((key, val))
			}
			else {
				None
			}
		}
	}
}

impl<'a> IntoIterator for &'a Dictionary<'a> {
	type Item     = (&'a str, &'a str);
	type IntoIter = DictionaryIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}
