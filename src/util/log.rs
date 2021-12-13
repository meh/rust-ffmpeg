use libc::{c_char, c_int, c_void};
use log;
use vsprintf::vsprintf;

use crate::ffi::*;

pub fn set_level(level: log::Level) {
	unsafe {
		av_log_set_level(match level {
			log::Level::Error => AV_LOG_ERROR,
			log::Level::Warn => AV_LOG_WARNING,
			log::Level::Info => AV_LOG_INFO,
			log::Level::Debug => AV_LOG_DEBUG,
			log::Level::Trace => AV_LOG_TRACE,
		});
	}
}

unsafe extern "C" fn callback(_ptr: *mut c_void, level: c_int, fmt: *const c_char, args: *mut __va_list_tag) {
	let string = vsprintf(fmt, args).unwrap();
	let level = match level {
		AV_LOG_PANIC | AV_LOG_FATAL | AV_LOG_ERROR => log::LevelFilter::Error,
		AV_LOG_WARNING => log::LevelFilter::Warn,
		AV_LOG_INFO => log::LevelFilter::Info,
		AV_LOG_VERBOSE | AV_LOG_DEBUG => log::LevelFilter::Debug,
		AV_LOG_TRACE => log::LevelFilter::Trace,
		_ => log::LevelFilter::Off,
	};

	if let Some(level) = level.to_level() {
		log::log!(target: "ffmpeg", level, "{}", string.trim());
	}
}

pub fn register() {
	unsafe {
		av_log_set_callback(Some(callback));
	}

	if let Some(level) = log::max_level().to_level() {
		set_level(level);
	}
}
