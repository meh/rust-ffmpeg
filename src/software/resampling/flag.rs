use libc::c_int;

use crate::ffi::*;

bitflags! {
		pub struct Flags: c_int {
				const FORCE = SWR_FLAG_RESAMPLE;
		}
}
