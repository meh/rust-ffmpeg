use crate::{format::Pixel, sys::*, util::frame, Error};

pub fn buffer_size(format: Pixel, width: u32, height: u32, align: u32) -> crate::Result<usize> {
	unsafe {
		match av_image_get_buffer_size(format.into(), width as i32, height as i32, align as i32) {
			e if e < 0 => Err(Error::from(e)),
			size => Ok(size as usize),
		}
	}
}

pub fn to_buffer(source: &frame::Video) -> crate::Result<Vec<u8>> {
	let mut buffer = Vec::new();
	into_buffer(source, &mut buffer)?;

	Ok(buffer)
}

pub fn into_buffer(source: &frame::Video, buffer: &mut Vec<u8>) -> crate::Result<()> {
	let size = buffer_size(source.format(), source.width(), source.height(), 1)?;
	buffer.resize(size, 0);

	let written = unsafe {
		match av_image_copy_to_buffer(
			buffer.as_mut_ptr(),
			size as i32,
			(*source.as_ptr()).data.as_ptr() as *mut _,
			(*source.as_ptr()).linesize.as_ptr() as *mut _,
			source.format().into(),
			source.width() as i32,
			source.height() as i32,
			1,
		) {
			e if e < 0 => Err(Error::from(e)),
			size => Ok(size),
		}
	}?;

	buffer.shrink_to(written as usize);

	Ok(())
}
