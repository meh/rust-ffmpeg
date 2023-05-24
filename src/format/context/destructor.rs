use crate::ffi::*;

#[derive(Copy, Clone, Debug)]
pub enum Mode {
	Input,
	Output,
}

pub struct Destructor {
	ptr: *mut AVFormatContext,
	mode: Mode,
}

impl Destructor {
	pub unsafe fn new(ptr: *mut AVFormatContext, mode: Mode) -> Self {
		Destructor { ptr, mode }
	}
}

impl Drop for Destructor {
	fn drop(&mut self) {
		unsafe {
			match self.mode {
				Mode::Input => avformat_close_input(&mut self.ptr),
				Mode::Output => {
					if (*self.ptr).flags & AVFMT_FLAG_CUSTOM_IO == 0
						&& (*self.ptr)
							.pb
							.as_ref()
							.map(|pb| !(*pb).av_class.is_null())
							.unwrap_or(false)
					{
						avio_close((*self.ptr).pb);
					}

					avformat_free_context(self.ptr);
				}
			}
		}
	}
}
