use std::{
	array,
	ffi::CString,
	fmt::{self, Display},
	hash::Hash,
	mem, ptr, slice,
	str::FromStr,
};

use thiserror::Error;

use crate::{
	ffi::{AVChannel, AVChannelOrder, *},
	Error,
};

// new channel layout since 5.1

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
#[cfg_attr(feature = "serde", derive(serde_derive::Serialize, serde_derive::Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_", rename_all = "kebab-case"))]
pub enum Channel {
	None = AVChannel::AV_CHAN_NONE.0,
	FrontLeft = AVChannel::AV_CHAN_FRONT_LEFT.0,
	FrontRight = AVChannel::AV_CHAN_FRONT_RIGHT.0,
	FrontCenter = AVChannel::AV_CHAN_FRONT_CENTER.0,
	LowFrequency = AVChannel::AV_CHAN_LOW_FREQUENCY.0,
	BackLeft = AVChannel::AV_CHAN_BACK_LEFT.0,
	BackRight = AVChannel::AV_CHAN_BACK_RIGHT.0,
	FrontLeftOfCenter = AVChannel::AV_CHAN_FRONT_LEFT_OF_CENTER.0,
	FrontRightOfCenter = AVChannel::AV_CHAN_FRONT_RIGHT_OF_CENTER.0,
	BackCenter = AVChannel::AV_CHAN_BACK_CENTER.0,
	SideLeft = AVChannel::AV_CHAN_SIDE_LEFT.0,
	SideRight = AVChannel::AV_CHAN_SIDE_RIGHT.0,
	TopCenter = AVChannel::AV_CHAN_TOP_CENTER.0,
	TopFrontLeft = AVChannel::AV_CHAN_TOP_FRONT_LEFT.0,
	TopFrontCenter = AVChannel::AV_CHAN_TOP_FRONT_CENTER.0,
	TopFrontRight = AVChannel::AV_CHAN_TOP_FRONT_RIGHT.0,
	TopBackLeft = AVChannel::AV_CHAN_TOP_BACK_LEFT.0,
	TopBackCenter = AVChannel::AV_CHAN_TOP_BACK_CENTER.0,
	TopBackRight = AVChannel::AV_CHAN_TOP_BACK_RIGHT.0,
	/// Stereo downmix.
	StereoLeft = AVChannel::AV_CHAN_STEREO_LEFT.0,
	/// Stereo downmix.
	StereoRight = AVChannel::AV_CHAN_STEREO_RIGHT.0,
	WideLeft = AVChannel::AV_CHAN_WIDE_LEFT.0,
	WideRight = AVChannel::AV_CHAN_WIDE_RIGHT.0,
	SurroundDirectLeft = AVChannel::AV_CHAN_SURROUND_DIRECT_LEFT.0,
	SurroundDirectRight = AVChannel::AV_CHAN_SURROUND_DIRECT_RIGHT.0,
	LowFrequency2 = AVChannel::AV_CHAN_LOW_FREQUENCY_2.0,
	TopSideLeft = AVChannel::AV_CHAN_TOP_SIDE_LEFT.0,
	TopSideRight = AVChannel::AV_CHAN_TOP_SIDE_RIGHT.0,
	BottomFrontCenter = AVChannel::AV_CHAN_BOTTOM_FRONT_CENTER.0,
	BottomFrontLeft = AVChannel::AV_CHAN_BOTTOM_FRONT_LEFT.0,
	BottomFrontRight = AVChannel::AV_CHAN_BOTTOM_FRONT_RIGHT.0,
	Unknown = AVChannel::AV_CHAN_UNKNOWN.0,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "serde", derive(serde_derive::Serialize, serde_derive::Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_", rename_all = "kebab-case"))]
pub enum ChannelOrder {
	Unspecified = AVChannelOrder::AV_CHANNEL_ORDER_UNSPEC.0,
	Native = AVChannelOrder::AV_CHANNEL_ORDER_NATIVE.0,
	Custom = AVChannelOrder::AV_CHANNEL_ORDER_CUSTOM.0,
	Ambisonic = AVChannelOrder::AV_CHANNEL_ORDER_AMBISONIC.0,
}

pub struct ChannelLayout(AVChannelLayout);

/// SAFETY: these are not auto-implemented due to the `opaque: *mut c_void` in
/// `AVChannelLayout`. The `ChannelLayout` wrapper does not expose `opaque`
/// directly, but only through `From` conversion into an `AVChannelLayout`,
/// which doesn't have these unsafe impls.
unsafe impl Sync for ChannelLayout {}
unsafe impl Send for ChannelLayout {}

type ChannelData = AVChannelLayout__bindgen_ty_1;

fn alloc_custom_channels(channels: &[CustomChannel]) -> *mut AVChannelCustom {
	unsafe {
		let map = av_malloc_array(channels.len(), mem::size_of::<AVChannelCustom>()).cast::<AVChannelCustom>();

		if map.is_null() {
			panic!("out of memory")
		}

		for (i, c) in channels.iter().enumerate() {
			*map.offset(i as isize) = c.0.clone();
		}

		map
	}
}

impl ChannelLayout {
	pub const fn new() -> Self {
		Self(AVChannelLayout {
			order: AVChannelOrder::AV_CHANNEL_ORDER_NATIVE,
			nb_channels: 0,
			u: ChannelData { mask: 0 },
			opaque: ptr::null_mut(),
		})
	}

	pub fn default(n_channels: i32) -> ChannelLayout {
		let mut layout = Self::new();

		unsafe {
			av_channel_layout_default(&mut layout.0 as *mut _, n_channels);
		}

		layout
	}

	pub fn from_name(name: impl Into<Vec<u8>>) -> Result<ChannelLayout, Error> {
		let s = CString::new(name.into()).map_err(|_| Error::InvalidData)?;
		let mut layout = Self::new();

		unsafe {
			match av_channel_layout_from_string(&mut layout.0 as *mut _, s.as_ptr()) {
				0 => Ok(layout),
				err => Err(Error::from(err)),
			}
		}
	}

	pub fn custom(channels: &[CustomChannel]) -> Self {
		assert!(channels.len() < i32::MAX as usize);

		Self(AVChannelLayout {
			order: AVChannelOrder::AV_CHANNEL_ORDER_CUSTOM,
			nb_channels: channels.len() as i32,
			u: ChannelData {
				map: alloc_custom_channels(channels),
			},
			opaque: ptr::null_mut(),
		})
	}

	const fn const_clone(&self) -> Self {
		Self(AVChannelLayout {
			order: self.0.order,
			nb_channels: self.0.nb_channels,
			u: self.0.u,
			opaque: self.0.opaque,
		})
	}

	pub const fn native(channels: &[Channel]) -> Self {
		Self::new().with_channels_native(channels)
	}

	pub const fn with_channels_native(&self, channels: &[Channel]) -> Self {
		let mut layout = self.const_clone();

		let mask = {
			let mut mask = 0;
			let mut idx = 0;
			while idx < channels.len() {
				let ch = channels[idx];
				if ch as u64 == Channel::None as u64 {
					continue;
				}

				mask |= 1 << ch as u64;
				idx += 1;
			}
			mask
		};

		unsafe {
			layout.0.u.mask |= mask;
			layout.0.nb_channels = layout.0.u.mask.count_ones() as i32;
		}

		layout
	}

	pub fn is_zeroed(&self) -> bool {
		unsafe {
			self.0.order == AVChannelOrder(0)
				&& self.0.nb_channels == 0
				&& self.0.u.mask == 0
				&& self.0.opaque == ptr::null_mut()
		}
	}

	fn contains_avchannel(&self, channel: AVChannel) -> Option<bool> {
		match self.0.order {
			AVChannelOrder::AV_CHANNEL_ORDER_NATIVE => unsafe { Some(self.0.u.mask & (1 << channel.0 as u64) != 0) },
			AVChannelOrder::AV_CHANNEL_ORDER_CUSTOM => unsafe {
				let channels = self.custom_channels_unchecked();
				Some(channels.iter().any(|ch| ch.0.id == channel))
			},

			// no information about channels available
			AVChannelOrder::AV_CHANNEL_ORDER_UNSPEC => None,
			AVChannelOrder::AV_CHANNEL_ORDER_AMBISONIC => None,

			order => panic!("invalid channel order: {order:?}"),
		}
	}

	/// Returns `Some(v)` if the membership of `channel` in `self` can be tested,
	/// where `v` is only true if `channel` is contained in `self`.
	/// For `ChannelOrder::Unspecified` and `ChannelOrder::Ambisonic`, this
	/// function currently returns `None`, though this may change.
	///
	/// Panics if called on a layout with an invalid channel order.
	pub fn contains(&self, channel: Channel) -> Option<bool> {
		self.contains_avchannel(AVChannel(channel as i32))
	}

	/// Similar to `contains`, check if all channels in `layout` are also
	/// contained in `self`. Only a few order combinations are supported:
	///
	/// - native to native
	/// -
	pub fn contains_all(&self, layout: &ChannelLayout) -> Option<bool> {
		match (self.0.order, layout.0.order) {
			(AVChannelOrder::AV_CHANNEL_ORDER_NATIVE, AVChannelOrder::AV_CHANNEL_ORDER_NATIVE) => unsafe {
				Some(self.0.u.mask & layout.0.u.mask == layout.0.u.mask)
			},

			// could be implemented in the future
			(AVChannelOrder::AV_CHANNEL_ORDER_CUSTOM, AVChannelOrder::AV_CHANNEL_ORDER_NATIVE) => None,

			(_, AVChannelOrder::AV_CHANNEL_ORDER_CUSTOM) => unsafe {
				let channels = self.custom_channels_unchecked();
				Some(
					channels
						.iter()
						.all(|ch| self.contains_avchannel(ch.0.id).unwrap_or(false)),
				)
			},

			// no information about channels available
			(AVChannelOrder::AV_CHANNEL_ORDER_UNSPEC, _) | (_, AVChannelOrder::AV_CHANNEL_ORDER_UNSPEC) => None,
			(AVChannelOrder::AV_CHANNEL_ORDER_AMBISONIC, _) | (_, AVChannelOrder::AV_CHANNEL_ORDER_AMBISONIC) => None,

			(self_order, layout_order) => panic!("invalid channel orders: {self_order:?}, {layout_order:?}"),
		}
	}

	// this would need only one pass with the bprint API, but that's currently
	// unwrapped
	pub fn describe(&self) -> Result<String, Error> {
		fn describe_into(buf: &mut [u8], layout: &ChannelLayout) -> Result<Result<String, usize>, Error> {
			unsafe {
				let bytes_needed = match av_channel_layout_describe(layout.as_ptr(), buf.as_mut_ptr() as *mut _, buf.len()) {
					e if e < 0 => return Err(Error::from(e))?,
					needed => needed as usize,
				};

				if bytes_needed <= buf.len() {
					let s = String::from_utf8_lossy(&buf[..bytes_needed]);
					Ok(Ok(s.trim_end_matches('\0').to_string()))
				} else {
					Ok(Err(bytes_needed))
				}
			}
		}

		const BUF_SIZE: usize = 64;
		let mut buf = [0u8; BUF_SIZE];

		match describe_into(&mut buf[..], self)? {
			Ok(s) => Ok(s),
			Err(needed) => {
				let mut buf = vec![0; needed + 1];

				Ok(describe_into(&mut buf[..], self)?.expect("allocated buffer should have been big enough"))
			}
		}
	}

	pub fn is_empty(&self) -> bool {
		self.0.nb_channels == 0
	}

	pub fn order(&self) -> ChannelOrder {
		match self.0.order {
			AVChannelOrder::AV_CHANNEL_ORDER_UNSPEC => ChannelOrder::Unspecified,
			AVChannelOrder::AV_CHANNEL_ORDER_NATIVE => ChannelOrder::Native,
			AVChannelOrder::AV_CHANNEL_ORDER_CUSTOM => ChannelOrder::Custom,
			AVChannelOrder::AV_CHANNEL_ORDER_AMBISONIC => ChannelOrder::Ambisonic,
			order => panic!("invalid channel order: {order:?}"),
		}
	}

	pub fn set_order(&mut self, order: ChannelOrder) {
		self.0.order = AVChannelOrder(order as u32);
	}

	pub fn channels(&self) -> i32 {
		self.0.nb_channels
	}

	pub fn as_ptr(&self) -> *const AVChannelLayout {
		&self.0 as *const _
	}

	pub fn as_mut_ptr(&mut self) -> *mut AVChannelLayout {
		&mut self.0
	}

	pub fn native_order_bits(&self) -> Option<u64> {
		(self.0.order == AVChannelOrder::AV_CHANNEL_ORDER_NATIVE).then_some(unsafe { self.0.u.mask })
	}

	unsafe fn custom_channels_unchecked(&self) -> &[CustomChannel] {
		slice::from_raw_parts(self.0.u.map.cast::<CustomChannel>(), self.0.nb_channels.max(0) as usize)
	}

	pub fn custom_channels(&self) -> Option<&[CustomChannel]> {
		(self.0.order == AVChannelOrder::AV_CHANNEL_ORDER_CUSTOM).then_some(unsafe { self.custom_channels_unchecked() })
	}

	pub fn is_valid(&self) -> bool {
		unsafe { av_channel_layout_check(&self.0) > 0 }
	}
}

impl ChannelLayout {
	pub const CUBE: ChannelLayout = Self::QUAD.with_channels_native(&[
		Channel::TopFrontLeft,
		Channel::TopFrontRight,
		Channel::TopBackLeft,
		Channel::TopBackRight,
	]);
	pub const HEXADECAGONAL: ChannelLayout = Self::OCTAGONAL.with_channels_native(&[
		Channel::WideLeft,
		Channel::WideRight,
		Channel::TopBackLeft,
		Channel::TopBackRight,
		Channel::TopBackCenter,
		Channel::TopFrontCenter,
		Channel::TopFrontLeft,
		Channel::TopFrontRight,
	]);
	pub const HEXAGONAL: ChannelLayout = Self::_5POINT0_BACK.with_channels_native(&[Channel::BackCenter]);
	pub const MONO: ChannelLayout = Self::native(&[Channel::FrontCenter]);
	pub const OCTAGONAL: ChannelLayout =
		Self::_5POINT0.with_channels_native(&[Channel::BackLeft, Channel::BackCenter, Channel::BackRight]);
	pub const QUAD: ChannelLayout = Self::STEREO.with_channels_native(&[Channel::BackLeft, Channel::BackRight]);
	pub const STEREO: ChannelLayout = Self::native(&[Channel::FrontLeft, Channel::FrontRight]);
	pub const STEREO_DOWNMIX: ChannelLayout = Self::native(&[Channel::StereoLeft, Channel::StereoRight]);
	pub const SURROUND: ChannelLayout = Self::STEREO.with_channels_native(&[Channel::FrontCenter]);
	pub const _22POINT2: ChannelLayout = Self::_5POINT1_BACK.with_channels_native(&[
		Channel::FrontLeftOfCenter,
		Channel::FrontRightOfCenter,
		Channel::BackCenter,
		Channel::LowFrequency2,
		Channel::SideLeft,
		Channel::SideRight,
		Channel::TopFrontLeft,
		Channel::TopFrontRight,
		Channel::TopFrontCenter,
		Channel::TopCenter,
		Channel::TopBackLeft,
		Channel::TopBackRight,
		Channel::TopSideLeft,
		Channel::TopSideRight,
		Channel::TopBackCenter,
		Channel::BottomFrontCenter,
		Channel::BottomFrontLeft,
		Channel::BottomFrontRight,
	]);
	pub const _2POINT1: ChannelLayout = Self::STEREO.with_channels_native(&[Channel::LowFrequency]);
	pub const _2_1: ChannelLayout = Self::STEREO.with_channels_native(&[Channel::BackCenter]);
	pub const _2_2: ChannelLayout = Self::STEREO.with_channels_native(&[Channel::SideLeft, Channel::SideRight]);
	pub const _3POINT1: ChannelLayout = Self::SURROUND.with_channels_native(&[Channel::LowFrequency]);
	pub const _4POINT0: ChannelLayout = Self::SURROUND.with_channels_native(&[Channel::BackCenter]);
	pub const _4POINT1: ChannelLayout = Self::_4POINT0.with_channels_native(&[Channel::LowFrequency]);
	pub const _5POINT0: ChannelLayout = Self::SURROUND.with_channels_native(&[Channel::SideLeft, Channel::SideRight]);
	pub const _5POINT0_BACK: ChannelLayout =
		Self::SURROUND.with_channels_native(&[Channel::BackLeft, Channel::BackRight]);
	pub const _5POINT1: ChannelLayout = Self::_5POINT0.with_channels_native(&[Channel::LowFrequency]);
	pub const _5POINT1_BACK: ChannelLayout = Self::_5POINT0_BACK.with_channels_native(&[Channel::LowFrequency]);
	pub const _6POINT0: ChannelLayout = Self::_5POINT0.with_channels_native(&[Channel::BackCenter]);
	pub const _6POINT0_FRONT: ChannelLayout =
		Self::_2_2.with_channels_native(&[Channel::FrontLeftOfCenter, Channel::FrontRightOfCenter]);
	pub const _6POINT1: ChannelLayout = Self::_5POINT1.with_channels_native(&[Channel::BackCenter]);
	pub const _6POINT1_BACK: ChannelLayout = Self::_5POINT1_BACK.with_channels_native(&[Channel::BackCenter]);
	pub const _6POINT1_FRONT: ChannelLayout = Self::_6POINT0_FRONT.with_channels_native(&[Channel::LowFrequency]);
	pub const _7POINT0: ChannelLayout = Self::_5POINT0.with_channels_native(&[Channel::BackLeft, Channel::BackRight]);
	pub const _7POINT0_FRONT: ChannelLayout =
		Self::_5POINT0.with_channels_native(&[Channel::FrontLeftOfCenter, Channel::FrontRightOfCenter]);
	pub const _7POINT1: ChannelLayout = Self::_5POINT1.with_channels_native(&[Channel::BackLeft, Channel::BackRight]);
	pub const _7POINT1_TOP_BACK: ChannelLayout =
		Self::_5POINT1_BACK.with_channels_native(&[Channel::TopFrontLeft, Channel::TopFrontRight]);
	pub const _7POINT1_WIDE: ChannelLayout =
		Self::_5POINT1.with_channels_native(&[Channel::FrontLeftOfCenter, Channel::FrontRightOfCenter]);
	pub const _7POINT1_WIDE_BACK: ChannelLayout =
		Self::_5POINT1_BACK.with_channels_native(&[Channel::FrontLeftOfCenter, Channel::FrontRightOfCenter]);
}

impl From<ChannelLayout> for AVChannelLayout {
	fn from(v: ChannelLayout) -> AVChannelLayout {
		v.0
	}
}

impl From<AVChannelLayout> for ChannelLayout {
	fn from(v: AVChannelLayout) -> ChannelLayout {
		Self(v)
	}
}

impl Hash for ChannelLayout {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.0.nb_channels.hash(state);
		self.0.order.hash(state);
		self.0.opaque.hash(state);
		unsafe {
			self.0.u.map.hash(state);
			self.0.u.mask.hash(state);
		}
	}
}

impl Clone for ChannelLayout {
	fn clone(&self) -> Self {
		let mut layout = ChannelLayout::new();
		unsafe { av_channel_layout_copy(layout.as_mut_ptr(), self.as_ptr()) };
		layout
	}
}

impl fmt::Debug for ChannelLayout {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut d = f.debug_struct("ChannelLayout");

		d.field("order", &self.0.order);
		d.field("nb_channels", &self.0.nb_channels);

		if let Some(custom) = self.custom_channels() {
			d.field("map", &custom);
		} else {
			unsafe {
				d.field("mask", &self.0.u.mask);
			}
		}

		d.field("opaque", &self.0.opaque);

		d.finish()
	}
}

impl PartialEq for ChannelLayout {
	fn eq(&self, other: &ChannelLayout) -> bool {
		unsafe {
			let ord = av_channel_layout_compare(self.as_ptr(), other.as_ptr());

			match ord {
				// negative return values for invalid layouts
				..=-1 => false,
				0 => true,
				1 => false,
				2.. => panic!("illegal return value"),
			}
		}
	}
}

impl Eq for ChannelLayout {}

impl Display for ChannelLayout {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.describe().unwrap_or_else(|_| String::from("unknown")))
	}
}

#[derive(Error, Debug)]
pub enum ParseChannelLayoutError {
	#[error("unknown format")]
	UnknownLayout,
}

impl FromStr for ChannelLayout {
	type Err = ParseChannelLayoutError;

	#[inline(always)]
	fn from_str(s: &str) -> Result<ChannelLayout, ParseChannelLayoutError> {
		ChannelLayout::from_name(s).map_err(|_| ParseChannelLayoutError::UnknownLayout)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct CustomChannel(AVChannelCustom);

impl CustomChannel {
	pub fn new(channel: Channel, name: Option<&str>) -> Self {
		Self::new_raw(channel as i32, name)
	}

	pub fn new_raw(channel: i32, name: Option<&str>) -> Self {
		let name = name.unwrap_or("").as_bytes();
		let mut name_with_zero = [0; 16];
		let len = name.len().min(15);
		name_with_zero[..len].copy_from_slice(&name[..len]);

		Self::custom(channel as i32, array::from_fn(|i| name_with_zero[i] as i8))
	}

	pub fn custom(channel: i32, name: [i8; 16]) -> Self {
		assert_eq!(name[15], 0);

		Self(AVChannelCustom {
			id: AVChannel(channel as i32),
			name,
			opaque: ptr::null_mut(),
		})
	}
}

impl From<Channel> for CustomChannel {
	fn from(v: Channel) -> CustomChannel {
		CustomChannel::new(v, None)
	}
}

impl From<CustomChannel> for AVChannelCustom {
	fn from(v: CustomChannel) -> AVChannelCustom {
		v.0
	}
}

impl From<AVChannelCustom> for CustomChannel {
	fn from(v: AVChannelCustom) -> CustomChannel {
		Self(v)
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use crate::ChannelLayout;

	#[test]
	fn mono() {
		let layout = ChannelLayout::default(1);
		let layout_string = layout.to_string();
		assert_eq!(layout_string, "mono");
		assert_eq!(layout, ChannelLayout::from_str(layout_string.as_str()).unwrap());
	}

	#[test]
	fn stereo() {
		let layout = ChannelLayout::default(2);
		let layout_string = layout.to_string();
		assert_eq!(layout_string, "stereo");
		assert_eq!(layout, ChannelLayout::from_str(layout_string.as_str()).unwrap());
	}

	#[test]
	fn six_point_zero() {
		let layout = ChannelLayout::from_name("6.0").unwrap();
		let layout_string = layout.to_string();
		assert_eq!(layout_string, "6.0");
		assert_eq!(layout, ChannelLayout::from_str(layout_string.as_str()).unwrap());
	}

	#[test]
	fn seven_point_one_wide() {
		let layout = ChannelLayout::from_name("7.1(wide)").unwrap();
		let layout_string = layout.to_string();
		assert_eq!(layout_string, "7.1(wide)");
		assert_eq!(layout, ChannelLayout::from_str(layout_string.as_str()).unwrap());
	}
}

#[cfg(feature = "serde")]
mod serde {
	//! It is expected that `CustomChannel::name` contains human-readable names in
	//! zero-terminated UTF-8. They are serialized as text instead of byte arrays
	//! to make them easily readable in e.g. JSON output. You'll need a different
	//! serde impl if you cleverly hid extra data after the null terminator, or
	//! use the name field to smuggle non-UTF-8 data.

	use std::{array, ffi::CStr, ptr, str};

	use serde_::{
		de::Error as _,
		ser::{Error as _, SerializeStruct},
		Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::{alloc_custom_channels, ChannelData, ChannelLayout, CustomChannel};
	use crate::ffi::{AVChannelLayout, AVChannelOrder};

	impl Serialize for CustomChannel {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let mut s = serializer.serialize_struct("CustomChannel", 2)?;
			s.serialize_field("id", &self.0.id.0)?;

			if self.0.name[0] != 0 {
				let u8_name = array::from_fn::<u8, 16, _>(|i| self.0.name[i] as u8);
				let str_name = CStr::from_bytes_until_nul(&u8_name[..])
					.map_err(|_| S::Error::custom("name is not a null-terminated string"))?
					.to_str()
					.map_err(|_| S::Error::custom("name is not valid UTF-8"))?;
				s.serialize_field("name", &str_name)?;
			}
			s.end()
		}
	}

	impl<'de> Deserialize<'de> for CustomChannel {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			#[derive(Deserialize)]
			#[serde(crate = "serde_")]
			struct Channel<'a> {
				id: i32,
				name: Option<&'a str>,
			}

			let Channel { id, name } = Channel::deserialize(deserializer)?;
			Ok(CustomChannel::new_raw(id, name.as_deref()))
		}
	}

	impl Serialize for ChannelLayout {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let mut s = serializer.serialize_struct("ChannelLayout", 2)?;

			// provide type hints in order to get compile-time errors if ffmpeg
			// changes the struct definition
			s.serialize_field::<u32>("order", &self.0.order.0)?;

			if let Some(custom) = self.custom_channels() {
				s.serialize_field("map", &custom)?;
			} else {
				s.serialize_field::<u64>("mask", unsafe { &self.0.u.mask })?;
			}

			s.end()
		}
	}

	impl<'de> Deserialize<'de> for ChannelLayout {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			#[derive(Deserialize)]
			#[serde(crate = "serde_")]
			struct OldLayout {
				bits: u64,
			}

			#[derive(Deserialize)]
			#[serde(crate = "serde_")]
			struct NewLayout {
				order: u32,

				mask: Option<u64>,
				map: Option<Vec<CustomChannel>>,
			}

			#[derive(Deserialize)]
			#[serde(untagged, crate = "serde_")]
			enum VersionedLayout {
				Old(OldLayout),
				New(NewLayout),
			}

			let (order, u, nb_channels);

			match VersionedLayout::deserialize(deserializer)? {
				VersionedLayout::Old(OldLayout { bits: mask }) => {
					order = AVChannelOrder::AV_CHANNEL_ORDER_NATIVE;
					u = ChannelData { mask };
					nb_channels = mask.count_ones() as i32;
				}
				VersionedLayout::New(NewLayout {
					order: num_order,
					mask,
					map,
				}) => {
					order = AVChannelOrder(num_order);

					match (order, mask, map) {
						(AVChannelOrder::AV_CHANNEL_ORDER_CUSTOM, _, Some(map)) => {
							u = ChannelData {
								map: alloc_custom_channels(&map),
							};
							nb_channels = map.len() as i32;
						}
						(
							AVChannelOrder::AV_CHANNEL_ORDER_UNSPEC
							| AVChannelOrder::AV_CHANNEL_ORDER_NATIVE
							| AVChannelOrder::AV_CHANNEL_ORDER_AMBISONIC,
							Some(mask),
							_,
						) => {
							u = ChannelData { mask };
							nb_channels = mask.count_ones() as i32
						}
						(_, _, _) => return Err(D::Error::missing_field("mask or map")),
					}
				}
			}

			Ok(ChannelLayout(AVChannelLayout {
				order,
				nb_channels,
				u,
				opaque: ptr::null_mut(),
			}))
		}
	}

	#[cfg(test)]
	mod tests {
		use std::fmt::Debug;

		use serde_::{de::DeserializeOwned, Serialize};

		use super::super::{Channel, ChannelLayout, CustomChannel};
		use crate::ffi::AVChannelOrder;

		fn round_trip_debug<T>(x: T)
		where
			T: Serialize + DeserializeOwned + Debug,
		{
			let json = serde_json::to_string(&x).unwrap();
			let y: T = serde_json::from_str(&json).unwrap();
			assert_eq!(format!("{x:?}"), format!("{y:?}"));
		}

		#[test]
		fn serde() {
			round_trip_debug(ChannelLayout::native(&[Channel::StereoRight, Channel::LowFrequency]));
			round_trip_debug(ChannelLayout::custom(&[
				CustomChannel::new(Channel::LowFrequency, Some("low-freq")),
				CustomChannel::new(Channel::BackCenter, None),
			]));
		}

		#[test]
		fn old_format() {
			let x: ChannelLayout = serde_json::from_str(r#"{ "bits": 31 }"#).unwrap();
			assert_eq!(x.0.order, AVChannelOrder::AV_CHANNEL_ORDER_NATIVE);
			assert_eq!(x.0.nb_channels, 5);
			assert_eq!(unsafe { x.0.u.mask }, 31);
		}
	}
}
