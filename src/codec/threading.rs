use libc::c_int;

use crate::ffi::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Config {
	pub kind: Type,
	pub count: usize,
}

impl Config {
	pub fn kind(value: Type) -> Self {
		Config {
			kind: value,
			..Default::default()
		}
	}

	pub fn count(value: usize) -> Self {
		Config {
			count: value,
			..Default::default()
		}
	}
}

impl Default for Config {
	fn default() -> Self {
		Config {
			kind: Type::None,
			count: 0,
		}
	}
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Type {
	None,
	Frame,
	Slice,
}

impl From<c_int> for Type {
	fn from(value: c_int) -> Type {
		match value {
			FF_THREAD_FRAME => Type::Frame,
			FF_THREAD_SLICE => Type::Slice,

			_ => Type::None,
		}
	}
}

impl Into<c_int> for Type {
	fn into(self) -> c_int {
		match self {
			Type::None => 0,
			Type::Frame => FF_THREAD_FRAME,
			Type::Slice => FF_THREAD_SLICE,
		}
	}
}
