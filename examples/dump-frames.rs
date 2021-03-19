use std::{env, fs::File, io::prelude::*};

use ffmpeg::{
	format::{input, Pixel},
	media::Type,
	software::scaling::{context::Context, flag::Flags},
	util::frame::video::Video,
};

fn main() -> Result<(), ffmpeg::Error> {
	ffmpeg::init().unwrap();

	if let Ok(mut ictx) = input(&env::args().nth(1).expect("Cannot open file.")) {
		let input = ictx
			.streams()
			.best(Type::Video)
			.ok_or(ffmpeg::Error::StreamNotFound)?;
		let video_stream_index = input.index();

		let mut decoder = input.codec().decoder().video()?;

		let mut scaler = Context::get(
			decoder.format(),
			decoder.width(),
			decoder.height(),
			Pixel::RGB24,
			decoder.width(),
			decoder.height(),
			Flags::BILINEAR,
		)?;

		let mut frame_index = 0;

		let mut receive_and_process_decoded_frames =
			|decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error> {
				let mut decoded = Video::empty();
				while decoder.receive_frame(&mut decoded).is_ok() {
					let mut rgb_frame = Video::empty();
					scaler.run(&decoded, &mut rgb_frame)?;
					save_file(&rgb_frame, frame_index).unwrap();
					frame_index += 1;
				}
				Ok(())
			};

		for res in ictx.packets() {
			let (stream, packet) = res?;
			if stream.index() == video_stream_index {
				decoder.send_packet(&packet)?;
				receive_and_process_decoded_frames(&mut decoder)?;
			}
		}
		decoder.send_eof()?;
		receive_and_process_decoded_frames(&mut decoder)?;
	}

	Ok(())
}

fn save_file(frame: &Video, index: usize) -> std::result::Result<(), std::io::Error> {
	let mut file = File::create(format!("frame{}.ppm", index))?;
	file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes())?;
	file.write_all(frame.data(0))?;
	Ok(())
}
