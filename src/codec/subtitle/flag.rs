use libc::c_int;

use crate::ffi::*;

bitflags! {
		pub struct Flags: c_int {
				const FORCED = AV_SUBTITLE_FLAG_FORCED;
		}
}
