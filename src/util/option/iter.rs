use sys::{av_opt_next, AVClass, AVOption};

pub struct AVOptionIterator {
	av_class: *const AVClass,
	option: *const AVOption,
}

impl AVOptionIterator {
	pub fn new(av_class: *const AVClass) -> Self {
		Self {
			av_class,
			option: std::ptr::null(),
		}
	}

	pub fn from_option(av_class: *const AVClass, option: *const AVOption) -> Self {
		Self { av_class, option }
	}

	pub fn class(&self) -> *const AVClass {
		self.av_class
	}
}

impl Iterator for AVOptionIterator {
	type Item = *const AVOption;

	fn next(&mut self) -> std::option::Option<<Self as Iterator>::Item> {
		unsafe {
			let priv_class = &self.av_class as *const *const AVClass;
			let ptr = av_opt_next(priv_class as *const std::ffi::c_void, self.option);

			if ptr.is_null() {
				None
			} else {
				self.option = ptr;

				Some(ptr)
			}
		}
	}
}
