use libc::{c_void, c_int};
use super::samplefmt::AVSampleFormat;

pub type AVAudioFifo = c_void;

extern {
	pub fn av_audio_fifo_free(af: *mut AVAudioFifo);
	pub fn av_audio_fifo_alloc(sample_fmt: AVSampleFormat, channels: c_int, nb_samples: c_int) -> *mut AVAudioFifo;
	pub fn av_audio_fifo_realloc(af: *mut AVAudioFifo, nb_samples: c_int) -> c_int;
	pub fn av_audio_fifo_write(af: *mut AVAudioFifo, data: *mut *mut c_void, nb_samples: c_int) -> c_int;
	pub fn av_audio_fifo_peek(af: *mut AVAudioFifo, data: *mut *mut c_void, nb_samples: c_int) -> c_int;
	pub fn av_audio_fifo_read(af: *mut AVAudioFifo, data: *mut *mut c_void, nb_samples: c_int) -> c_int;
	pub fn av_audio_fifo_drain(af: *mut AVAudioFifo, nb_samples: c_int) -> c_int;
	pub fn av_audio_fifo_reset(af: *mut AVAudioFifo);
	pub fn av_audio_fifo_size(af: *mut AVAudioFifo) -> c_int;
	pub fn av_audio_fifo_space(af: *mut AVAudioFifo) -> c_int;
}
