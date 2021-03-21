use super::Context;
use crate::{decoder, frame, util::format, ChannelLayout, Error};

impl frame::Audio {
	#[inline]
	pub fn resampler(
		&self,
		format: format::Sample,
		channel_layout: ChannelLayout,
		rate: u32,
	) -> Result<Context, Error> {
		Context::get(
			self.format(),
			self.channel_layout(),
			self.sample_rate(),
			format,
			channel_layout,
			rate,
		)
	}
}

impl decoder::Audio {
	#[inline]
	pub fn resampler(
		&self,
		format: format::Sample,
		channel_layout: ChannelLayout,
		rate: u32,
	) -> Result<Context, Error> {
		Context::get(
			self.format(),
			self.channel_layout(),
			self.sample_rate(),
			format,
			channel_layout,
			rate,
		)
	}
}
