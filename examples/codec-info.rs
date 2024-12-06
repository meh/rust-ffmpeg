use std::env;
use ffmpeg::Codec;

fn print_codec(codec: Codec) {
	println!("\t id: {:?}", codec.id());
	println!("\t name: {}", codec.name());
	println!("\t description: {}", codec.description());
	println!("\t medium: {:?}", codec.medium());
	println!("\t capabilities: {:?}", codec.capabilities());

	if let Some(profiles) = codec.profiles() {
		println!("\t profiles: {:?}", profiles.collect::<Vec<_>>());
	}
	else {
		println!("\t profiles: none");
	}

	if let Ok(video) = codec.video() {
		if let Some(rates) = video.frame_rates() {
			println!("\t frame rates: {:?}", rates.collect::<Vec<_>>());
		}
		else {
			println!("\t frame rates: any");
		}

		if let Some(formats) = video.formats() {
			println!("\t formats: {:?}", formats.collect::<Vec<_>>());
		}
		else {
			println!("\t formats: any");
		}
	}

	if let Ok(audio) = codec.audio() {
		if let Some(rates) = audio.sample_rates() {
			println!("\t sample rates: {:?}", rates.collect::<Vec<_>>());
		}
		else {
			println!("\t sample rates: any");
		}

		if let Some(formats) = audio.formats() {
			println!("\t formats: {:?}", formats.collect::<Vec<_>>());
		}
		else {
			println!("\t formats: any");
		}

        #[cfg(feature = "ffmpeg_6_0")]
        {
		    if let Some(layouts) = audio.channel_layouts() {
			    println!("\t channel_layouts:");
			    for layout in layouts {
				    println!("\t\t {}", layout.describe().unwrap());
			    }
		    }
		    else {
			    println!("\t channel_layouts: any");
		    }
        }
	}

	println!("\t max_lowres: {:?}", codec.max_lowres());
}

fn main() {
	ffmpeg::init().unwrap();

	for arg in env::args().skip(1) {
		if let Some(codec) = ffmpeg::decoder::find_by_name(&arg) {
			println!("type: decoder");
			print_codec(codec);
		}

		if let Some(codec) = ffmpeg::encoder::find_by_name(&arg) {
			println!();
			println!("type: encoder");
			print_codec(codec);
		}
	}
}
