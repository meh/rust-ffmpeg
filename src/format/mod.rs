pub use crate::util::format::{pixel, sample, Pixel, Sample};
use crate::util::interrupt;

pub mod io;
pub mod stream;
pub use self::stream::{Stream, StreamMut};

pub mod chapter;

pub mod context;
pub use self::context::Context;

pub mod format;
pub use self::format::{all, flag, input, output, Flags, Input, Output};

pub mod network;

use std::{
	ffi::{CStr, OsStr},
	ptr,
	str::from_utf8_unchecked,
};

use crate::{ffi::*, util::from_os_str, Dictionary, Error, Format};

pub fn version() -> u32 {
	unsafe { avformat_version() }
}

pub fn configuration() -> &'static str {
	unsafe { from_utf8_unchecked(CStr::from_ptr(avformat_configuration()).to_bytes()) }
}

pub fn license() -> &'static str {
	unsafe { from_utf8_unchecked(CStr::from_ptr(avformat_license()).to_bytes()) }
}

pub fn open(path_or_url: impl AsRef<OsStr>, format: &Format) -> Result<Context, Error> {
	unsafe {
		let mut ps = ptr::null_mut();
		let path = from_os_str(path_or_url);

		match *format {
			Format::Input(ref format) => {
				match avformat_open_input(&mut ps, path.as_ptr(), format.as_ptr() as *mut _, ptr::null_mut()) {
					0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
						r if r >= 0 => Ok(Context::Input(context::Input::wrap(ps))),
						e => Err(Error::from(e)),
					},

					e => Err(Error::from(e)),
				}
			}

			Format::Output(ref format) => {
				match avformat_alloc_output_context2(&mut ps, format.as_ptr() as *mut _, ptr::null(), path.as_ptr()) {
					0 => {
						let output = context::Output::wrap(ps);
						if output.format().flags().contains(Flags::NO_FILE) {
							return Ok(Context::Output(output));
						}
						match avio_open(&mut (*ps).pb, path.as_ptr(), AVIO_FLAG_WRITE) {
							0 => Ok(Context::Output(output)),
							e => Err(Error::from(e)),
						}
					}

					e => Err(Error::from(e)),
				}
			}
		}
	}
}

pub fn open_with(path_or_url: impl AsRef<OsStr>, format: &Format, options: Dictionary) -> Result<Context, Error> {
	unsafe {
		let mut ps = ptr::null_mut();
		let path = from_os_str(path_or_url);

		match *format {
			Format::Input(ref format) => {
				let mut opts = options.disown();
				let res = avformat_open_input(&mut ps, path.as_ptr(), format.as_ptr() as *mut _, &mut opts);
				Dictionary::own(opts);

				match res {
					0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
						r if r >= 0 => Ok(Context::Input(context::Input::wrap(ps))),
						e => Err(Error::from(e)),
					},

					e => Err(Error::from(e)),
				}
			}

			Format::Output(ref format) => {
				match avformat_alloc_output_context2(&mut ps, format.as_ptr(), ptr::null(), path.as_ptr()) {
					0 => {
						let output = context::Output::wrap(ps);
						if output.format().flags().contains(Flags::NO_FILE) {
							return Ok(Context::Output(output));
						}
						match avio_open(&mut (*ps).pb, path.as_ptr(), AVIO_FLAG_WRITE) {
							0 => Ok(Context::Output(output)),
							e => Err(Error::from(e)),
						}
					}

					e => Err(Error::from(e)),
				}
			}
		}
	}
}

pub fn input(path_or_url: impl AsRef<OsStr>) -> Result<context::Input, Error> {
	unsafe {
		let mut ps = ptr::null_mut();
		let path = from_os_str(path_or_url);

		match avformat_open_input(&mut ps, path.as_ptr(), ptr::null(), ptr::null_mut()) {
			0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
				r if r >= 0 => Ok(context::Input::wrap(ps)),
				e => {
					avformat_close_input(&mut ps);
					Err(Error::from(e))
				}
			},

			e => Err(Error::from(e)),
		}
	}
}

pub fn input_with_dictionary(path_or_url: impl AsRef<OsStr>, options: Dictionary) -> Result<context::Input, Error> {
	unsafe {
		let mut ps = ptr::null_mut();
		let path = from_os_str(path_or_url);
		let mut opts = options.disown();
		let res = avformat_open_input(&mut ps, path.as_ptr(), ptr::null(), &mut opts);

		Dictionary::own(opts);

		match res {
			0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
				r if r >= 0 => Ok(context::Input::wrap(ps)),
				e => {
					avformat_close_input(&mut ps);
					Err(Error::from(e))
				}
			},

			e => Err(Error::from(e)),
		}
	}
}

pub fn input_with_interrupt<P: AsRef<OsStr>>(
	path_or_url: P,
	closure: impl FnMut() -> bool,
) -> Result<context::Input, Error> {
	unsafe {
		let mut ps = avformat_alloc_context();
		let path = from_os_str(path_or_url);
		(*ps).interrupt_callback = interrupt::new(Box::new(closure)).interrupt;

		match avformat_open_input(&mut ps, path.as_ptr(), ptr::null(), ptr::null_mut()) {
			0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
				r if r >= 0 => Ok(context::Input::wrap(ps)),
				e => {
					avformat_close_input(&mut ps);
					Err(Error::from(e))
				}
			},

			e => Err(Error::from(e)),
		}
	}
}

pub fn output(path_or_url: impl AsRef<OsStr>) -> Result<context::Output, Error> {
	unsafe {
		let mut ps = ptr::null_mut();
		let path = from_os_str(path_or_url);

		match avformat_alloc_output_context2(&mut ps, ptr::null(), ptr::null(), path.as_ptr()) {
			0 => {
				let output = context::Output::wrap(ps);
				if output.format().flags().contains(Flags::NO_FILE) {
					return Ok(output);
				}
				match avio_open(&mut (*ps).pb, path.as_ptr(), AVIO_FLAG_WRITE) {
					0 => Ok(output),
					e => Err(Error::from(e)),
				}
			}

			e => Err(Error::from(e)),
		}
	}
}

pub fn output_with(path_or_url: impl AsRef<OsStr>, options: Dictionary) -> Result<context::Output, Error> {
	unsafe {
		let mut ps = ptr::null_mut();
		let path = from_os_str(path_or_url);

		match avformat_alloc_output_context2(&mut ps, ptr::null(), ptr::null(), path.as_ptr()) {
			0 => {
				let output = context::Output::wrap(ps);
				if output.format().flags().contains(Flags::NO_FILE) {
					return Ok(output);
				}

				let mut opts = options.disown();
				let res = avio_open2(&mut (*ps).pb, path.as_ptr(), AVIO_FLAG_WRITE, ptr::null(), &mut opts);
				Dictionary::own(opts);

				match res {
					0 => Ok(output),
					e => Err(Error::from(e)),
				}
			}

			e => Err(Error::from(e)),
		}
	}
}

pub fn output_as(path_or_url: impl AsRef<OsStr>, format: Output) -> Result<context::Output, Error> {
	unsafe {
		let mut ps = ptr::null_mut();
		let path = from_os_str(path_or_url);

		match avformat_alloc_output_context2(&mut ps, format.as_ptr(), ptr::null_mut(), path.as_ptr()) {
			0 => {
				let output = context::Output::wrap(ps);
				if output.format().flags().contains(Flags::NO_FILE) {
					return Ok(output);
				}
				match avio_open(&mut (*ps).pb, path.as_ptr(), AVIO_FLAG_WRITE) {
					0 => Ok(output),
					e => Err(Error::from(e)),
				}
			}

			e => Err(Error::from(e)),
		}
	}
}

pub fn output_as_with(
	path_or_url: impl AsRef<OsStr>,
	format: Output,
	options: Dictionary,
) -> Result<context::Output, Error> {
	unsafe {
		let mut ps = ptr::null_mut();
		let path = from_os_str(path_or_url);

		match avformat_alloc_output_context2(&mut ps, format.as_ptr(), ptr::null_mut(), path.as_ptr()) {
			0 => {
				let output = context::Output::wrap(ps);
				if output.format().flags().contains(Flags::NO_FILE) {
					return Ok(output);
				}

				let mut opts = options.disown();
				let res = avio_open2(&mut (*ps).pb, path.as_ptr(), AVIO_FLAG_WRITE, ptr::null(), &mut opts);
				Dictionary::own(opts);

				match res {
					0 => Ok(output),
					e => Err(Error::from(e)),
				}
			}

			e => Err(Error::from(e)),
		}
	}
}
