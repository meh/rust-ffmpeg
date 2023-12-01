use std::{env, fs::File};

use ffmpeg::format::{input, output};

fn main() {
	env_logger::init();
	ffmpeg::init().unwrap();

	input::all().for_each(|input| {
		println!("\n\ninput: {}\n", input.name());
		input.options().for_each(|option| {
			println!(
				"{}({:?} - {:?}): {:?}",
				option.name(),
				option.kind(),
				option.default_value(),
				option.help()
			);
			option.constants().for_each(|asdf| {
				println!(" {}({:?}) - {:?}", asdf.name(), asdf.default_value(), asdf.help());
			})
		});

		output::all().for_each(|output| {
			println!("\n\noutput: {}\n", output.name());
			output.options().for_each(|option| {
				println!(
					"{}({:?} - {:?}): {:?}",
					option.name(),
					option.kind(),
					option.default_value(),
					option.help()
				);
				option.constants().for_each(|asdf| {
					println!(" {}({:?}) - {:?}", asdf.name(), asdf.default_value(), asdf.help());
				})
			});
		});
	});
}
