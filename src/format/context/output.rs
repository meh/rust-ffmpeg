use std::{
	ffi::CString,
	mem::size_of,
	ops::{Deref, DerefMut},
	ptr,
};

use libc;

use super::{common::Context, destructor};
use crate::{
	ffi::*,
	format::{self, io::Io},
	ChapterMut, Dictionary, Error, Rational, StreamMut,
};

pub struct Output {
	ptr: *mut AVFormatContext,
	ctx: Context,
	// keeps IO proxy alive, no way to drop it otherwise
	_io: Option<Io>,
}

unsafe impl Send for Output {}

impl Output {
	pub unsafe fn wrap(ptr: *mut AVFormatContext) -> Self {
		Output {
			ptr,
			ctx: Context::wrap(ptr, destructor::Mode::Output),
			_io: None,
		}
	}

	pub unsafe fn wrap_with(ptr: *mut AVFormatContext, io: Io) -> Self {
		Output {
			ptr,
			ctx: Context::wrap(ptr, destructor::Mode::Output),
			_io: Some(io),
		}
	}

	pub unsafe fn as_ptr(&self) -> *const AVFormatContext {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFormatContext {
		self.ptr
	}
}

impl Output {
	pub fn format(&self) -> format::Output {
		unsafe { format::Output::wrap((*self.as_ptr()).oformat) }
	}

	pub fn write_header(&mut self) -> Result<(), Error> {
		unsafe {
			match avformat_write_header(self.as_mut_ptr(), ptr::null_mut()) {
				0 => Ok(()),
				e => Err(Error::from(e)),
			}
		}
	}

	pub fn write_header_with(&mut self, options: Dictionary) -> Result<Dictionary, Error> {
		unsafe {
			let mut opts = options.disown();
			let res = avformat_write_header(self.as_mut_ptr(), &mut opts);

			match res {
				0 => Ok(Dictionary::own(opts)),
				e => Err(Error::from(e)),
			}
		}
	}

	pub fn write_trailer(&mut self) -> Result<(), Error> {
		unsafe {
			// Although the documentation doesn't specify positive return values [0],
			// av_write_trailer can return the positive avio position (from e.g.
			// mov_write_mfra_tag), which ffmpeg usually ignores, and only tests for return
			// values smaller than 0 instead [1].
			//
			// [0]: https://github.com/FFmpeg/FFmpeg/blob/3ac23440ef4a5a203f53b33325fa38b2e8afa219/libavformat/avformat.h#L2474
			// [1]: https://github.com/FFmpeg/FFmpeg/search?q=av_write_trailer
			match av_write_trailer(self.as_mut_ptr()) {
				e if e < 0 => Err(Error::from(e)),
				_ => Ok(()),
			}
		}
	}

	pub fn flush(&mut self) -> Result<(), Error> {
		unsafe {
			match av_write_frame(self.as_mut_ptr(), ptr::null_mut()) {
				e if e < 0 => Err(Error::from(e)),
				_ => Ok(()),
			}
		}
	}

	pub fn add_stream(&mut self) -> Result<StreamMut<'_>, Error> {
		unsafe {
			let ptr = avformat_new_stream(self.as_mut_ptr(), ptr::null());

			if ptr.is_null() {
				return Err(Error::Unknown);
			}

			let index = (*self.ctx.as_ptr()).nb_streams - 1;

			Ok(StreamMut::wrap(&mut self.ctx, index as usize))
		}
	}

	pub fn add_chapter<R: Into<Rational>, S: AsRef<str>>(
		&mut self,
		id: i64,
		time_base: R,
		start: i64,
		end: i64,
		title: S,
	) -> Result<ChapterMut<'_>, Error> {
		// avpriv_new_chapter is private (libavformat/internal.h)

		if start > end {
			return Err(Error::InvalidData);
		}

		let mut existing = None;
		for chapter in self.chapters() {
			if chapter.id() == id {
				existing = Some(chapter.index());
				break;
			}
		}

		let index = match existing {
			Some(index) => index,
			None => unsafe {
				let ptr = av_mallocz(size_of::<AVChapter>()).as_mut().ok_or(Error::Bug)?;
				let mut nb_chapters = (*self.as_ptr()).nb_chapters as i32;

				// chapters array will be freed by `avformat_free_context`
				av_dynarray_add(
					&mut (*self.as_mut_ptr()).chapters as *mut _ as *mut libc::c_void,
					&mut nb_chapters,
					ptr,
				);

				if nb_chapters > 0 {
					(*self.as_mut_ptr()).nb_chapters = nb_chapters as u32;
					let index = (*self.ctx.as_ptr()).nb_chapters - 1;
					index as usize
				}
				else {
					// failed to add the chapter
					av_freep(ptr);
					return Err(Error::Bug);
				}
			},
		};

		let mut chapter = self.chapter_mut(index).ok_or(Error::Bug)?;

		chapter.set_id(id);
		chapter.set_time_base(time_base);
		chapter.set_start(start);
		chapter.set_end(end);
		chapter.set_metadata("title", title);

		Ok(chapter)
	}

	pub fn set_metadata(&mut self, dictionary: Dictionary) {
		unsafe {
			(*self.as_mut_ptr()).metadata = dictionary.disown();
		}
	}
}

impl Deref for Output {
	type Target = Context;

	fn deref(&self) -> &Self::Target {
		&self.ctx
	}
}

impl DerefMut for Output {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.ctx
	}
}

pub fn dump(ctx: &Output, index: i32, url: Option<&str>) {
	let url = url.map(|u| CString::new(u).unwrap());

	unsafe {
		av_dump_format(
			ctx.as_ptr() as *mut _,
			index,
			url.unwrap_or_else(|| CString::new("").unwrap()).as_ptr(),
			1,
		);
	}
}
