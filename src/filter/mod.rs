pub mod flag;
pub use self::flag::Flags;

pub mod pad;
pub use self::pad::Pad;

pub mod filter;
pub use self::filter::Filter;

pub mod context;
pub use self::context::{Context, Sink, Source};

pub mod graph;
use std::{
	ffi::{CStr, CString},
	str::from_utf8_unchecked,
};

pub use self::graph::Graph;
use crate::ffi::*;

pub fn version() -> u32 {
	unsafe { avfilter_version() }
}

pub fn configuration() -> &'static str {
	unsafe { from_utf8_unchecked(CStr::from_ptr(avfilter_configuration()).to_bytes()) }
}

pub fn license() -> &'static str {
	unsafe { from_utf8_unchecked(CStr::from_ptr(avfilter_license()).to_bytes()) }
}

pub fn find(name: &str) -> Option<Filter> {
	unsafe {
		let name = CString::new(name).unwrap();
		let ptr = avfilter_get_by_name(name.as_ptr());

		if ptr.is_null() {
			None
		}
		else {
			Some(Filter::wrap(ptr))
		}
	}
}
