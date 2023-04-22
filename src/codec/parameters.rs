use std::{any::Any, ops::Deref, rc::Rc};

use super::{decoder, encoder, Context, Decoder, Encoder, Id};
use crate::{ffi::*, media, Error};

pub struct Parameters {
	ptr: *mut AVCodecParameters,
	owner: Option<Rc<dyn Any>>,
}

unsafe impl Send for Parameters {}

impl Parameters {
	pub unsafe fn wrap(ptr: *mut AVCodecParameters, owner: Option<Rc<dyn Any>>) -> Self {
		Parameters { ptr, owner }
	}

	pub unsafe fn as_ptr(&self) -> *const AVCodecParameters {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVCodecParameters {
		self.ptr
	}
}

impl Parameters {
	pub fn new() -> Self {
		unsafe {
			Parameters {
				ptr: avcodec_parameters_alloc(),
				owner: None,
			}
		}
	}

	pub fn medium(&self) -> media::Type {
		unsafe { media::Type::from((*self.as_ptr()).codec_type) }
	}

	pub fn set_medium(&mut self, value: media::Type) -> &mut Self {
		unsafe {
			(*self.as_mut_ptr()).codec_type = value.into();
		}
		self
	}

	pub fn id(&self) -> Id {
		unsafe { Id::from((*self.as_ptr()).codec_id) }
	}

	pub fn set_id(&mut self, value: Id) -> &mut Self {
		unsafe {
			(*self.as_mut_ptr()).codec_id = value.into();
		}
		self
	}

	pub fn tag(&self) -> u32 {
		unsafe { (*self.as_ptr()).codec_tag }
	}

	pub fn set_tag(&mut self, value: u32) -> &mut Self {
		unsafe {
			(*self.as_mut_ptr()).codec_tag = value;
		}
		self
	}

	pub fn encoder(&self) -> Result<Encoder, Error> {
		let codec = encoder::find(self.id()).ok_or(Error::EncoderNotFound)?;
		let mut ctx = Encoder::new(codec)?;
		ctx.set_parameters(self.clone())?;

		Ok(ctx)
	}

	/// Also see [`Stream::decoder`](crate::format::Stream::decoder)
	pub fn decoder(&self) -> Result<Decoder, Error> {
		let codec = decoder::find(self.id()).ok_or(Error::DecoderNotFound)?;
		let mut ctx = Decoder::new(codec)?;
		ctx.set_parameters(self.clone())?;

		Ok(ctx)
	}
}

impl Default for Parameters {
	fn default() -> Self {
		Self::new()
	}
}

impl Drop for Parameters {
	fn drop(&mut self) {
		unsafe {
			if self.owner.is_none() {
				avcodec_parameters_free(&mut self.as_mut_ptr());
			}
		}
	}
}

impl Clone for Parameters {
	fn clone(&self) -> Self {
		let mut ctx = Parameters::new();
		ctx.clone_from(self);

		ctx
	}

	fn clone_from(&mut self, source: &Self) {
		unsafe {
			avcodec_parameters_copy(self.as_mut_ptr(), source.as_ptr());
		}
	}
}

impl<C: Deref<Target = Context>> From<C> for Parameters {
	fn from(context: C) -> Parameters {
		let mut parameters = Parameters::new();
		let context = context.deref();
		unsafe {
			avcodec_parameters_from_context(parameters.as_mut_ptr(), context.as_ptr());
		}
		parameters
	}
}
