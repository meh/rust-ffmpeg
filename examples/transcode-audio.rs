use std::{env, path::Path};

use ffmpeg::{codec, filter, format, frame, media, rescale, Rescale};
#[cfg(feature = "ffmpeg_6_0")]
use ffmpeg::channel_layout::{self, ChannelLayout};

fn filter(
	spec: &str,
	decoder: &codec::decoder::Audio,
	encoder: &codec::encoder::Audio,
) -> Result<filter::Graph, ffmpeg::Error> {
	let mut filter = filter::Graph::new();

    #[cfg(feature = "ffmpeg_6_0")]
    let channel_layout_arg = format!(":channel_layout={}",
		decoder.channel_layout().describe().unwrap());

    // Not yet implemented for older versions
    #[cfg(not(feature = "ffmpeg_6_0"))]
    let channel_layout_arg = "";

	let args = format!(
		"time_base={}:sample_rate={}:sample_fmt={}{}",
		decoder.time_base().unwrap(),
		decoder.sample_rate(),
		decoder.format().name(),
		channel_layout_arg
	);

	filter.add(&filter::find("abuffer").unwrap(), "in", &args)?;
	filter.add(&filter::find("abuffersink").unwrap(), "out", "")?;

	{
		let mut out = filter.get("out").unwrap();

		out.set_sample_format(encoder.format());
		#[cfg(feature = "ffmpeg_6_0")]
		out.set_channel_layout(encoder.channel_layout());
		out.set_sample_rate(encoder.sample_rate());
	}

	filter.output("in", 0)?.input("out", 0)?.parse(spec)?;
	filter.validate()?;

	println!("{}", filter.dump());

	if let Some(codec) = encoder.codec() {
		if !codec
			.capabilities()
			.contains(ffmpeg::codec::capabilities::Capabilities::VARIABLE_FRAME_SIZE)
		{
			filter.get("out").unwrap().sink().set_frame_size(encoder.frame_size());
		}
	}

	Ok(filter)
}

struct Transcoder {
	stream: usize,
	filter: filter::Graph,
	decoder: codec::decoder::Audio,
	encoder: codec::encoder::Audio,
	in_time_base: ffmpeg::Rational,
	out_time_base: ffmpeg::Rational,
}

fn transcoder<P: AsRef<Path>>(
	ictx: &mut format::context::Input,
	octx: &mut format::context::Output,
	path: &P,
	filter_spec: &str,
) -> Result<Transcoder, ffmpeg::Error> {
	let input = ictx
		.streams()
		.best(media::Type::Audio)
		.expect("could not find best audio stream");
	let mut decoder = input.decoder()?.audio()?;
	let codec = ffmpeg::encoder::find(octx.format().codec(path, media::Type::Audio))
		.expect("failed to find encoder")
		.audio()?;
	let global = octx
		.format()
		.flags()
		.contains(ffmpeg::format::flag::Flags::GLOBAL_HEADER);

	decoder.set_parameters(input.parameters())?;

	let mut output = octx.add_stream()?;
	let mut encoder = codec::Encoder::new(codec)?.audio()?;

	if global {
		encoder.set_flags(ffmpeg::codec::flag::Flags::GLOBAL_HEADER);
	}
	encoder.set_sample_rate(decoder.sample_rate());
	encoder.set_format(codec.formats().expect("unknown supported formats").next().unwrap());
	encoder.set_bit_rate(decoder.bit_rate());
	encoder.set_max_bit_rate(decoder.max_bit_rate());

	#[cfg(feature = "ffmpeg_6_0")]
	{
		let channel_layout = codec
			.channel_layouts()
			.map(|cls| cls.best(decoder.channel_layout().channels()))
			.unwrap_or(ChannelLayout::STEREO);

		encoder.set_channel_layout(channel_layout);
		encoder.set_channels(channel_layout.channels());
	}

	let enc_tb = (1, decoder.sample_rate() as i32);
	encoder.set_time_base(Some(enc_tb));
	output.set_time_base(Some(enc_tb));

	let encoder = encoder.open_as(codec)?;
	output.set_parameters(encoder.parameters());

	let filter = filter(filter_spec, &decoder, &encoder)?;

	let in_time_base = decoder.time_base().unwrap();
	let out_time_base = output.time_base().unwrap();

	Ok(Transcoder {
		stream: input.index(),
		filter,
		decoder,
		encoder,
		in_time_base,
		out_time_base,
	})
}

impl Transcoder {
	fn send_frame_to_encoder(&mut self, frame: &ffmpeg::Frame) {
		self.encoder.send_frame(frame).unwrap();
	}

	fn send_eof_to_encoder(&mut self) {
		self.encoder.send_eof().unwrap();
	}

	fn receive_and_process_encoded_packets(&mut self, octx: &mut format::context::Output) {
		let mut encoded = ffmpeg::Packet::empty();
		while self.encoder.receive_packet(&mut encoded).is_ok() {
			encoded.set_stream(0);
			encoded.rescale_ts(self.in_time_base, self.out_time_base);
			encoded.write_interleaved(octx).unwrap();
		}
	}

	fn add_frame_to_filter(&mut self, frame: &ffmpeg::Frame) {
		self.filter.get("in").unwrap().source().add(frame).unwrap();
	}

	fn flush_filter(&mut self) {
		self.filter.get("in").unwrap().source().flush().unwrap();
	}

	fn get_and_process_filtered_frames(&mut self, octx: &mut format::context::Output) {
		let mut filtered = frame::Audio::empty();
		while self.filter.get("out").unwrap().sink().frame(&mut filtered).is_ok() {
			self.send_frame_to_encoder(&filtered);
			self.receive_and_process_encoded_packets(octx);
		}
	}

	fn send_packet_to_decoder(&mut self, packet: &ffmpeg::Packet) {
		self.decoder.send_packet(packet).unwrap();
	}

	fn send_eof_to_decoder(&mut self) {
		self.decoder.send_eof().unwrap();
	}

	fn receive_and_process_decoded_frames(&mut self, octx: &mut format::context::Output) {
		let mut decoded = frame::Audio::empty();
		while self.decoder.receive_frame(&mut decoded).is_ok() {
			let timestamp = decoded.timestamp();
			decoded.set_pts(timestamp);
			self.add_frame_to_filter(&decoded);
			self.get_and_process_filtered_frames(octx);
		}
	}
}

// Transcode the `best` audio stream of the input file into a the output file
// while applying a given filter. If no filter was specified the stream gets
// copied (`anull` filter).
//
// Example 1: Transcode *.mp3 file to *.wmv while speeding it up
// transcode-audio in.mp3 out.wmv "atempo=1.2"
//
// Example 2: Overlay an audio file
// transcode-audio in.mp3 out.mp3 "amovie=overlay.mp3 [ov]; [in][ov] amerge
// [out]"
//
// Example 3: Seek to a specified position (in seconds)
// transcode-audio in.mp3 out.mp3 anull 30
fn main() {
	ffmpeg::init().unwrap();

	let input = env::args().nth(1).expect("missing input");
	let output = env::args().nth(2).expect("missing output");
	let filter = env::args().nth(3).unwrap_or_else(|| "anull".to_owned());
	let seek = env::args().nth(4).and_then(|s| s.parse::<i64>().ok());

	let mut ictx = format::input(&input).unwrap();
	let mut octx = format::output(&output).unwrap();
	let mut transcoder = transcoder(&mut ictx, &mut octx, &output, &filter).unwrap();

	if let Some(position) = seek {
		// If the position was given in seconds, rescale it to ffmpegs base timebase.
		let position = position.rescale((1, 1), rescale::TIME_BASE);
		// If this seek was embedded in the transcoding loop, a call of `flush()`
		// for every opened buffer after the successful seek would be advisable.
		ictx.seek(position, ..position).unwrap();
	}

	octx.set_metadata(ictx.metadata().to_owned());
	octx.write_header().unwrap();

	for res in ictx.packets() {
		let (stream, mut packet) = res.unwrap();
		if stream.index() == transcoder.stream {
			packet.rescale_ts(stream.time_base().unwrap(), transcoder.in_time_base);
			transcoder.send_packet_to_decoder(&packet);
			transcoder.receive_and_process_decoded_frames(&mut octx);
		}
	}

	transcoder.send_eof_to_decoder();
	transcoder.receive_and_process_decoded_frames(&mut octx);

	transcoder.flush_filter();
	transcoder.get_and_process_filtered_frames(&mut octx);

	transcoder.send_eof_to_encoder();
	transcoder.receive_and_process_encoded_packets(&mut octx);

	octx.write_trailer().unwrap();
}
