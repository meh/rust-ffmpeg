extern crate ffmpeg;

use std::env;
use std::path::Path;
use std::error::Error;

use self::ffmpeg::{format, codec, frame, media};

struct FilterGraph<'a> {
	buffersrc: ffmpeg::filter::Context<'a>,
	buffersink: ffmpeg::filter::Context<'a>,
	filter_graph: ffmpeg::filter::FilterGraph<'a>,
}

fn build_filter_graph<'a>(spec: &str, decoder: &codec::decoder::Audio, encoder: &codec::encoder::Audio) -> Result<FilterGraph<'a>, ffmpeg::Error> {
	let mut filter_graph = try!(ffmpeg::filter::FilterGraph::new());

	let time_base_q = ffmpeg::Rational(1, 1000000);
	let args = format!("time_base={}:sample_rate={}:sample_fmt={}:channel_layout=0x{:x}",
					   time_base_q, decoder.rate(), decoder.format().name(),
					   decoder.channel_layout().bits());

	let mut buffersrc = try!(filter_graph.create_filter_by_name("abuffer", "in", &args));
	let mut buffersink = try!(filter_graph.create_filter_by_name("abuffersink", "out", ""));

	buffersink.set_sample_format(encoder.format());
	buffersink.set_channel_layout(encoder.channel_layout());
	buffersink.set_sample_rate(encoder.rate());

	let output = try!(ffmpeg::filter::InOut::new("in", &mut buffersrc, 0));
	let input = try!(ffmpeg::filter::InOut::new("out", &mut buffersink, 0));

	try!(filter_graph.parse_ptr(spec, input, output));
	try!(filter_graph.config());

	Ok(FilterGraph {
		buffersrc: buffersrc,
		buffersink: buffersink,
		filter_graph: filter_graph,
	})
}

struct AudioTranscoder<'a> {
	input_stream_index: usize,
	filter_graph: FilterGraph<'a>,
	decoder: codec::decoder::Audio,
	encoder: codec::encoder::audio::Encoder,
}

fn build_transcoder<'a>(ictx: &mut format::Context, octx: &mut format::Context) -> Result<AudioTranscoder<'a>, ffmpeg::Error> {
	let streams = ictx.streams();
	let input_stream = streams.best(media::Type::Audio)
		.ok_or("Could not find best audio stream")
		.unwrap();

	let decoder = try!(input_stream.codec().decoder().audio());
	let generic_encoder_codec = ffmpeg::encoder::find(decoder.id())
		.ok_or("Failed do find encoder codec")
		.unwrap();
	let encoder_codec = try!(generic_encoder_codec.audio());
	let mut output_stream = octx.new_stream(&encoder_codec)
		.ok_or("Failed to allocate output stream")
		.unwrap();
	output_stream.set_time_base(input_stream.time_base());
	let mut encoder = try!(output_stream.codec().encoder().audio());

	encoder.set_rate(decoder.rate() as i32);
	encoder.set_channel_layout(decoder.channel_layout());
	encoder.set_channels(decoder.channel_layout().channels());
	encoder.set_format(encoder_codec.formats().and_then(|mut f| f.next())
					   .ok_or("No encoder formats found").unwrap());
	output_stream.set_time_base((1, decoder.rate() as i32));

	let filter_graph = try!(build_filter_graph("anull", &decoder, &encoder));
	let opened_encoder = try!(encoder.open_as(&encoder_codec));

	Ok(AudioTranscoder {
		input_stream_index: input_stream.index(),
		filter_graph: filter_graph,
		decoder: decoder,
		encoder: opened_encoder,
	})
}

fn main() {
	if let Ok(()) = ffmpeg::init() {
		let mut ictx = env::args().nth(1)
			.ok_or("Please supply an input file.".to_string())
			.and_then(|file| ffmpeg::format::open_input(&Path::new(&file))
					  .map_err(|e| e.description().to_string()))
			.unwrap();

		let mut octx = env::args().nth(2)
			.ok_or("Please supply an output file.".to_string())
			.and_then(|file| format::open_output(&Path::new(&file))
					  .map_err(|e| e.description().to_string()))
			.unwrap();

		let mut transcoder = build_transcoder(&mut ictx, &mut octx).unwrap();

		octx.write_header().unwrap();

		let time_base_q = (1, 1000000);

		for (stream, mut packet) in ictx.packets() {
			if stream.index() == transcoder.input_stream_index {
				let (os_index, os_time_base) = {
					let os = octx.stream(stream.index()).unwrap();
					(os.index(), os.time_base())
				};

				packet.rescale_ts(stream.time_base(), time_base_q);
				let mut frame = frame::Audio::empty();

				if transcoder.decoder.decode(&packet, &mut frame).unwrap() {
					// we ignore the pts here. There are several issues why:
					// - Some internet sources state that it is deprecated (or will be soon).
					// - It also works without it (for mp3).
					transcoder.filter_graph.buffersrc.add_frame(&mut frame).unwrap();
					for filtered in transcoder.filter_graph.buffersink.frames() {
						let mut encoded = ffmpeg::Packet::empty();
						transcoder.encoder.encode(&filtered.into(), &mut encoded).unwrap();
						encoded.set_stream(os_index);
						encoded.rescale_ts(time_base_q, os_time_base);
						encoded.write_interleaved(&mut octx).unwrap();
					}
				}
			}
		}

		octx.write_trailer().unwrap();
	}
}
