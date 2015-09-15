use libc::{c_void, c_char, c_uchar, c_int, c_uint, c_ulong, uint8_t, int64_t, uint64_t, size_t, SEEK_CUR};
use super::super::avutil::{AVClass, AVBPrint, AVDictionary};

pub const AVIO_SEEKABLE_NORMAL: c_int = 0x0001;

pub const AVSEEK_SIZE:  c_int = 0x10000;
pub const AVSEEK_FORCE: c_int = 0x20000;

pub const AVIO_FLAG_READ:       c_int = 1;
pub const AVIO_FLAG_WRITE:      c_int = 2;
pub const AVIO_FLAG_READ_WRITE: c_int = AVIO_FLAG_READ | AVIO_FLAG_WRITE;
pub const AVIO_FLAG_NONBLOCK:   c_int = 8;
pub const AVIO_FLAG_DIRECT:     c_int = 0x8000;

#[repr(C)]
pub struct AVIOInterruptCB {
	pub callback: extern fn(*mut c_void) -> c_int,
	pub opaque: *mut c_void,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVIODirEntryType {
	AVIO_ENTRY_UNKNOWN,
	AVIO_ENTRY_BLOCK_DEVICE,
	AVIO_ENTRY_CHARACTER_DEVICE,
	AVIO_ENTRY_DIRECTORY,
	AVIO_ENTRY_NAMED_PIPE,
	AVIO_ENTRY_SYMBOLIC_LINK,
	AVIO_ENTRY_SOCKET,
	AVIO_ENTRY_FILE,
	AVIO_ENTRY_SERVER,
	AVIO_ENTRY_SHARE,
	AVIO_ENTRY_WORKGROUP,
}

#[repr(C)]
pub struct AVIODirEntry {
	name: *mut c_char,
	kind: c_int,
	utf8: c_int,

	size: int64_t,
	modification_timestamp: int64_t,

	access_timestamp: int64_t,

	status_change_timestamp: int64_t,

	user_id: int64_t,
	group_id: int64_t,
	filemode: int64_t,
}

#[repr(C)]
pub struct AVIODirContext {
	url_context: *mut c_void,
}

#[repr(C)]
pub struct AVIOContext {
	pub av_class: *mut AVClass,
	pub buffer: *mut c_uchar,
	pub buffer_size: c_int,
	pub buf_ptr: *mut c_uchar,
	pub buf_end: *mut c_uchar,
	pub opaque: *mut c_void,
	pub read_packet: extern fn(*mut c_void, *mut uint8_t, c_int) -> c_int,
	pub write_packet: extern fn(*mut c_void, *mut uint8_t, c_int) -> c_int,
	pub seek: extern fn(*mut c_void, int64_t, c_int) -> int64_t,
	pub pos: int64_t,
	pub must_flush: c_int,
	pub eof_reached: c_int,
	pub write_flag: c_int,
	pub max_packet_size: c_int,
	pub checksum: c_ulong,
	pub checksum_ptr: *mut c_uchar,
	pub update_checksum: extern fn(c_ulong, *const uint8_t, c_uint) -> c_uint,
	pub error: c_int,
	pub read_pause: extern fn(*mut c_void, c_int) -> c_int,
	pub read_seek: extern fn(*mut c_void, c_int, int64_t, c_int) -> int64_t,
	pub seekable: c_int,
	pub maxsize: int64_t,
	pub direct: c_int,
	pub bytes_read: int64_t,
	pub seek_count: c_int,
	pub writeout_count: c_int,
	pub orig_buffer_size: c_int,
	pub short_seek_threshold: c_int,
}

#[inline(always)]
pub unsafe fn avio_tell(s: *mut AVIOContext) -> int64_t {
	avio_seek(s, 0, SEEK_CUR)
}

extern {
	pub fn avio_find_protocol_name(url: *const c_char) -> *const c_char;
	pub fn avio_check(url: *const c_char, flags: c_int) -> c_int;
	pub fn avio_open_dir(s: *mut *mut AVIODirContext, url: *const c_char, options: *mut *mut AVDictionary) -> c_int;
	pub fn avio_read_dir(s: *mut AVIODirContext, next: *mut *mut AVIODirEntry) -> c_int;
	pub fn avio_close_dir(s: *mut *mut AVIODirContext) -> c_int;
	pub fn avio_free_directory_entry(entry: *mut *mut AVIODirEntry);
	pub fn avio_alloc_context(buffer: *mut c_uchar, buffer_size: c_int, write_flag: c_int, opaque: *mut c_void, read_packet: extern fn(*mut c_void, *mut uint8_t, c_int) -> c_int, write_packet: extern fn(*mut c_void, *mut uint8_t, c_int) -> c_int, seek: extern fn(*mut c_void, int64_t, c_int) -> int64_t) -> *mut AVIOContext;

	pub fn avio_write(s: *mut AVIOContext, buf: *const c_uchar, size: c_int);
	pub fn avio_w8(s: *mut AVIOContext, b: c_int);
	pub fn avio_wl64(s: *mut AVIOContext, val: uint64_t);
	pub fn avio_wb64(s: *mut AVIOContext, val: uint64_t);
	pub fn avio_wl32(s: *mut AVIOContext, val: c_uint);
	pub fn avio_wb32(s: *mut AVIOContext, val: c_uint);
	pub fn avio_wl24(s: *mut AVIOContext, val: c_uint);
	pub fn avio_wb24(s: *mut AVIOContext, val: c_uint);
	pub fn avio_wl16(s: *mut AVIOContext, val: c_uint);
	pub fn avio_wb16(s: *mut AVIOContext, val: c_uint);

	pub fn avio_put_str(s: *mut AVIOContext, string: *const c_char) -> c_int;
	pub fn avio_put_str16le(s: *mut AVIOContext, string: *const c_char) -> c_int;
	pub fn avio_put_str16be(s: *mut AVIOContext, string: *const c_char) -> c_int;

	pub fn avio_seek(s: *mut AVIOContext, offset: int64_t, whence: c_int) -> int64_t;
	pub fn avio_skip(s: *mut AVIOContext, offset: int64_t) -> int64_t;
	pub fn avio_size(s: *mut AVIOContext) -> int64_t;
	pub fn avio_feof(s: *mut AVIOContext) -> c_int;

	pub fn avio_printf(s: *mut AVIOContext, fmt: *const c_char, ...) -> c_int;
	pub fn avio_flush(s: *mut AVIOContext);

	pub fn avio_read(s: *mut AVIOContext, buf: *mut c_uchar, size: c_int) -> c_int;
	pub fn avio_r8(s: *mut AVIOContext) -> c_int;
	pub fn avio_rl16(s: *mut AVIOContext) -> c_uint;
	pub fn avio_rl24(s: *mut AVIOContext) -> c_uint;
	pub fn avio_rl32(s: *mut AVIOContext) -> c_uint;
	pub fn avio_rl64(s: *mut AVIOContext) -> uint64_t;
	pub fn avio_rb16(s: *mut AVIOContext) -> c_uint;
	pub fn avio_rb24(s: *mut AVIOContext) -> c_uint;
	pub fn avio_rb32(s: *mut AVIOContext) -> c_uint;
	pub fn avio_rb64(s: *mut AVIOContext) -> uint64_t;

	pub fn avio_get_str(pb: *mut AVIOContext, maxlen: c_int, buf: *mut c_char, buflen: c_int) -> c_int;
	pub fn avio_get_str16le(pb: *mut AVIOContext, maxlen: c_int, buf: *mut c_char, buflen: c_int) -> c_int;
	pub fn avio_get_str16be(pb: *mut AVIOContext, maxlen: c_int, buf: *mut c_char, buflen: c_int) -> c_int;

	pub fn avio_open(s: *mut *mut AVIOContext, url: *const c_char, flags: c_int) -> c_int;
	pub fn avio_open2(s: *mut *mut AVIOContext, url: *const c_char, flags: c_int, int_cb: *const AVIOInterruptCB, options: *mut *mut AVDictionary) -> c_int;

	pub fn avio_close(s: *mut AVIOContext) -> c_int;
	pub fn avio_closep(s: *mut *mut AVIOContext) -> c_int;

	pub fn avio_open_dyn_buf(s: *mut *mut AVIOContext) -> c_int;
	pub fn avio_close_dyn_buf(s: *mut AVIOContext, pbuffer: *mut *mut uint8_t) -> c_int;

	pub fn avio_enum_protocols(opaque: *mut *mut c_void, output: c_int) -> *const c_char;
	pub fn avio_pause(h: *mut AVIOContext, pause: c_int) -> c_int;
	pub fn avio_seek_time(h: *mut AVIOContext, stream_index: c_int, timestamp: int64_t, flags: c_int) -> int64_t;
	pub fn avio_read_to_bprint(h: *mut AVIOContext, pb: *mut AVBPrint, max_size: size_t) -> c_int;

	pub fn avio_accept(s: *mut AVIOContext, c: *mut *mut AVIOContext) -> c_int;
	pub fn avio_handshake(c: *mut AVIOContext) -> c_int;

	pub fn avpriv_io_move(url_src: *const c_char, url_dst: *const c_char) -> c_int;
	pub fn avpriv_io_delete(url: *const c_char) -> c_int;
}
