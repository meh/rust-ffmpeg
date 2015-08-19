use std::marker::PhantomData;
use std::ptr;
use std::ffi::{CStr, CString};

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

pub struct DictionaryIter<'a> {
	ptr: *const AVDictionary,
	cur: *mut AVDictionaryEntry,

	_marker: PhantomData<&'a Dictionary<'a>>,
}

impl<'a> DictionaryIter<'a> {
	pub fn new(dictionary: *const AVDictionary) -> Self {
		DictionaryIter { ptr: dictionary, cur: ptr::null_mut(), _marker: PhantomData }
	}
}

impl<'a> Iterator for DictionaryIter<'a> {
	type Item = (String, String);

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		let empty = CString::new("").unwrap();
		let entry = unsafe {
			av_dict_get(self.ptr, empty.as_ptr(), self.cur, AV_DICT_IGNORE_SUFFIX)
		};
		if !entry.is_null() {
			let key = unsafe {
				CStr::from_ptr((*entry).key).to_string_lossy().into_owned()
			};
			let val = unsafe {
				CStr::from_ptr((*entry).value).to_string_lossy().into_owned()
			};
			self.cur = entry;
			Some((key, val))
		} else { None }
	}
}
