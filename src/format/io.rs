use std::{
	convert::TryInto,
	io::{self, Read, Seek, SeekFrom, Write},
	ptr, slice,
};

use libc::{c_int, c_void, EINVAL, SEEK_CUR, SEEK_END, SEEK_SET};

use crate::{
	ffi::*,
	format::{context, format},
	Error,
};

pub enum Proxy {
	Input(Box<dyn InputIo>),
	Stream(Box<dyn StreamIo>),
	Output(Box<dyn Write>),
}

impl Proxy {
	pub fn as_read(&mut self) -> &mut dyn Read {
		match self {
			Proxy::Input(ref mut inner) => inner,
			Proxy::Stream(ref mut inner) => inner,
			Proxy::Output(_) => unreachable!(),
		}
	}

	pub fn as_seek(&mut self) -> &mut dyn Seek {
		match self {
			Proxy::Input(ref mut inner) => inner,
			Proxy::Stream(_) | Proxy::Output(_) => unreachable!(),
		}
	}

	pub fn as_write(&mut self) -> &mut dyn Write {
		match self {
			Proxy::Output(ref mut inner) => inner,
			Proxy::Stream(_) | Proxy::Input(_) => unreachable!(),
		}
	}
}

#[derive(Debug)]
pub struct Io {
	ptr: *mut AVIOContext,
	proxy: *mut Proxy,
}

pub trait InputIo: Read + Seek {}
impl<T: Read + Seek> InputIo for T {}

pub trait StreamIo: Read {}
impl<T: Read> StreamIo for T {}

pub trait OutputIo: Write {}
impl<T: Write> OutputIo for T {}

impl Io {
	pub unsafe fn as_ptr(&self) -> *const AVIOContext {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVIOContext {
		self.ptr
	}
}

fn as_error(err: io::Error) -> c_int {
	if let Some(value) = err.raw_os_error() {
		value.try_into().unwrap()
	}
	else {
		EINVAL
	}
}

unsafe extern "C" fn read_packet(opaque: *mut c_void, buf: *mut u8, size: c_int) -> c_int {
	let mut proxy = Box::<Proxy>::from_raw(opaque.cast());
	let buffer = slice::from_raw_parts_mut(buf, size.try_into().unwrap());

	let result = match proxy.as_read().read(buffer) {
		Ok(0) => AVERROR_EOF,
		Ok(size) => size.try_into().unwrap(),
		Err(err) => as_error(err),
	};

	Box::into_raw(proxy);
	result.try_into().unwrap()
}

// copy of unstable `Seek::stream_len`
// https://github.com/rust-lang/rust/issues/59359
fn stream_len(mut seek: impl Seek) -> io::Result<u64> {
	let old_pos = seek.stream_position()?;
	let len = seek.seek(SeekFrom::End(0))?;

	// Avoid seeking a third time when we were already at the end of the
	// stream. The branch is usually way cheaper than a seek operation.
	if old_pos != len {
		seek.seek(SeekFrom::Start(old_pos))?;
	}

	Ok(len)
}

unsafe extern "C" fn seek(opaque: *mut c_void, offset: i64, whence: c_int) -> i64 {
	let mut proxy = Box::<Proxy>::from_raw(opaque.cast());

	let result: i64 = if whence == AVSEEK_SIZE {
		match stream_len(proxy.as_seek()) {
			Ok(size) => size.try_into().unwrap(),
			Err(err) => as_error(err).into(),
		}
	}
	else {
		let whence = match whence {
			SEEK_SET => SeekFrom::Start(offset.try_into().unwrap()),
			SEEK_END => SeekFrom::End(offset.try_into().unwrap()),
			SEEK_CUR => SeekFrom::Current(offset.try_into().unwrap()),
			_ => unreachable!(),
		};

		match proxy.as_seek().seek(whence) {
			Ok(size) => size.try_into().unwrap(),
			Err(err) => as_error(err).into(),
		}
	};

	Box::into_raw(proxy);
	result
}

unsafe extern "C" fn write_packet(opaque: *mut c_void, buf: *mut u8, size: c_int) -> c_int {
	let mut proxy = Box::<Proxy>::from_raw(opaque.cast());
	let buffer = slice::from_raw_parts(buf, size.try_into().unwrap());

	let result: c_int = match proxy.as_write().write(buffer) {
		Ok(size) => size.try_into().unwrap(),
		Err(err) => as_error(err),
	};

	Box::into_raw(proxy);
	result
}

impl Io {
	pub fn input(value: impl Read + Seek + 'static) -> Self {
		unsafe {
			let proxy = Box::into_raw(Box::new(Proxy::Input(Box::new(value))));
			let ptr = avio_alloc_context(
				ptr::null_mut(),
				0,
				AVIO_FLAG_READ & AVIO_FLAG_DIRECT,
				proxy.cast(),
				Some(read_packet),
				None,
				Some(seek),
			);

			Io { proxy, ptr }
		}
	}

	pub fn stream(value: impl Read + 'static) -> Self {
		unsafe {
			let proxy = Box::into_raw(Box::new(Proxy::Stream(Box::new(value))));
			let ptr = avio_alloc_context(
				ptr::null_mut(),
				0,
				AVIO_FLAG_READ & AVIO_FLAG_DIRECT,
				proxy.cast(),
				Some(read_packet),
				None,
				None,
			);

			Io { proxy, ptr }
		}
	}

	pub fn output(value: impl Write + 'static) -> Self {
		unsafe {
			let proxy = Box::into_raw(Box::new(Proxy::Output(Box::new(value))));
			let buffer = av_malloc(4096);
			let ptr = avio_alloc_context(
				buffer.cast(),
				4096,
				AVIO_FLAG_WRITE,
				proxy.cast(),
				None,
				Some(write_packet),
				None,
			);

			Io { proxy, ptr }
		}
	}
}

impl Drop for Io {
	fn drop(&mut self) {
		unsafe {
			if !(*self.ptr).buffer.is_null() {
				av_free((*self.ptr).buffer.cast());
			}

			avio_context_free(&mut self.ptr);
			Box::from_raw(self.proxy);
		}
	}
}

pub fn input(io: impl Read + Seek + 'static) -> Result<context::Input, Error> {
	unsafe {
		let mut ps = avformat_alloc_context();
		let mut io = Io::input(io);
		(*ps).pb = io.as_mut_ptr();

		match avformat_open_input(&mut ps, ptr::null_mut(), ptr::null_mut(), ptr::null_mut()) {
			0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
				r if r >= 0 => Ok(context::Input::wrap_with(ps, io)),

				e => {
					avformat_close_input(&mut ps);
					Err(Error::from(e))
				}
			},

			e => Err(Error::from(e)),
		}
	}
}

pub fn input_as(io: impl Read + Seek + 'static, format: format::Input) -> Result<context::Input, Error> {
	unsafe {
		let mut ps = avformat_alloc_context();
		let mut io = Io::input(io);
		(*ps).pb = io.as_mut_ptr();

		match avformat_open_input(&mut ps, ptr::null_mut(), format.as_ptr(), ptr::null_mut()) {
			0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
				r if r >= 0 => Ok(context::Input::wrap_with(ps, io)),

				e => {
					avformat_close_input(&mut ps);
					Err(Error::from(e))
				}
			},

			e => Err(Error::from(e)),
		}
	}
}

pub fn output(io: impl Write + 'static, format: format::Output) -> Result<context::Output, Error> {
	unsafe {
		let mut ps = ptr::null_mut();
		let mut io = Io::output(io);

		match avformat_alloc_output_context2(&mut ps, format.as_ptr(), ptr::null_mut(), ptr::null_mut()) {
			n if n >= 0 => {
				(*ps).pb = io.as_mut_ptr();
				Ok(context::Output::wrap_with(ps, io))
			}

			e => Err(Error::from(e)),
		}
	}
}
