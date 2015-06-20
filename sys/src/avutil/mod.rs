#[macro_use]
mod macros;

mod adler32;
pub use self::adler32::*;

mod aes;
pub use self::aes::*;

mod audio_fifo;
pub use self::audio_fifo::*;

mod string;
pub use self::string::*;
pub use self::string::AVEscapeMode::*;

mod base64;
pub use self::base64::*;

mod blowfish;
pub use self::blowfish::*;

mod bprint;
pub use self::bprint::*;

mod buffer;
pub use self::buffer::*;

mod camellia;
pub use self::camellia::*;

mod cast5;
pub use self::cast5::*;

mod channel_layout;
pub use self::channel_layout::*;
pub use self::channel_layout::AVMatrixEncoding::*;

mod cpu;
pub use self::cpu::*;

mod crc;
pub use self::crc::*;
pub use self::crc::AVCRCId::*;

mod dict;
pub use self::dict::*;

mod display;
pub use self::display::*;

mod downmix_info;
pub use self::downmix_info::*;

mod error;
pub use self::error::*;

mod eval;
pub use self::eval::*;

mod fifo;
pub use self::fifo::*;

mod file;
pub use self::file::*;

mod frame;
pub use self::frame::*;
pub use self::frame::AVFrameSideDataType::*;

mod hash;
pub use self::hash::*;

mod hmac;
pub use self::hmac::*;
pub use self::hmac::AVHMACType::*;

mod imgutils;
pub use self::imgutils::*;

mod lfg;
pub use self::lfg::*;

mod log;
pub use self::log::*;
pub use self::log::AVClassCategory::*;

mod lzo;
pub use self::lzo::*;

mod mathematics;
pub use self::mathematics::*;
pub use self::mathematics::AVRounding::*;

mod md5;
pub use self::md5::*;

mod mem;
pub use self::mem::*;

mod motion_vector;
pub use self::motion_vector::*;

mod murmur3;
pub use self::murmur3::*;

mod opt;
pub use self::opt::*;
pub use self::opt::AVOptionType::*;

mod parseutils;
pub use self::parseutils::*;

mod pixdesc;
pub use self::pixdesc::*;

mod pixfmt;
pub use self::pixfmt::*;
pub use self::pixfmt::AVPixelFormat::*;
pub use self::pixfmt::default::*;
pub use self::pixfmt::AVColorPrimaries::*;
pub use self::pixfmt::AVColorTransferCharacteristic::*;
pub use self::pixfmt::AVColorSpace::*;
pub use self::pixfmt::AVColorRange::*;
pub use self::pixfmt::AVChromaLocation::*;

mod random_seed;
pub use self::random_seed::*;

mod rational;
pub use self::rational::*;

mod replaygain;
pub use self::replaygain::*;

mod ripemd;
pub use self::ripemd::*;

mod samplefmt;
pub use self::samplefmt::*;
pub use self::samplefmt::AVSampleFormat::*;

mod sha;
pub use self::sha::*;

mod sha512;
pub use self::sha512::*;

mod stereo3d;
pub use self::stereo3d::*;
pub use self::stereo3d::AVStereo3DType::*;

mod threadmessage;
pub use self::threadmessage::*;

mod time;
pub use self::time::*;

mod timecode;
pub use self::timecode::*;

mod twofish;
pub use self::twofish::*;

mod xtea;
pub use self::xtea::*;

mod util;
pub use self::util::*;
pub use self::util::AVMediaType::*;
pub use self::util::AVPictureType::*;

#[cfg_attr(feature = "static", link(name = "avutil", kind = "static"))]
#[cfg_attr(not(feature = "static"), link(name = "avutil"))]
extern { }
