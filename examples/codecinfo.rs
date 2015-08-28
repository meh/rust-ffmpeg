extern crate ffmpeg;

use std::env;

fn print_codec_info(typee: &str, codec: &ffmpeg::Codec) {
	println!("type: {}", typee);
	println!("\t id: {:?}", codec.id());
	println!("\t name: {}", codec.name());
	println!("\t description: {}", codec.description());
	println!("\t medium: {:?}", codec.medium());
	println!("\t framerates: {:?}", codec.framerates().collect::<Vec<_>>());
	println!("\t pixel_formats: {:?}", codec.pixel_formats().collect::<Vec<_>>());
	println!("\t samplerates: {:?}", codec.samplerates().collect::<Vec<_>>());
	println!("\t sample_formats: {:?}", codec.sample_formats().collect::<Vec<_>>());
	println!("\t channel_layouts: {:?}", codec.channel_layouts().collect::<Vec<_>>());
	println!("\t max_lowres: {:?}", codec.max_lowres());
}

fn main() {
	if let Ok(()) = ffmpeg::init() {
		for arg in env::args().skip(1) {
			if let Some(codec) = ffmpeg::decoder::find_by_name(&arg) {
				print_codec_info("decoder", &codec);
			}
			if let Some(codec) = ffmpeg::encoder::find_by_name(&arg) {
				print_codec_info("encoder", &codec);
			}
		}
	}
}
