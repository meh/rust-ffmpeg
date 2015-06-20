use libc::{c_int, uint8_t, uint32_t, size_t};

pub type AVCRC = uint32_t;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVCRCId {
	AV_CRC_8_ATM,
	AV_CRC_16_ANSI,
	AV_CRC_16_CCITT,
	AV_CRC_32_IEEE,
	AV_CRC_32_IEEE_LE,
	AV_CRC_16_ANSI_LE,
	AV_CRC_24_IEEE = 12,
	AV_CRC_MAX,
}

extern {
	pub fn av_crc_init(ctx: *mut AVCRC, le: c_int, bits: c_int, poly: uint32_t, ctx_size: c_int) -> c_int;
	pub fn av_crc_get_table(crc_id: AVCRCId) -> *const AVCRC;
	pub fn av_crc(ctx: *const AVCRC, crc: uint32_t, buffer: *const uint8_t, length: size_t) -> uint32_t;
}
