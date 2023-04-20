use ffmpeg_sys::SWR_FLAG_RESAMPLE;
use libc::c_int;

bitflags! {
	#[cfg_attr(feature = "serde", derive(serde_derive::Serialize, serde_derive::Deserialize))]
	#[cfg_attr(feature = "serde", serde(crate = "serde_", rename_all = "kebab-case"))]
	pub struct Flags: c_int {
		const FORCE = SWR_FLAG_RESAMPLE;
	}
}
