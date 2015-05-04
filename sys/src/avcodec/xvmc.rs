use libc::{c_void, c_short, c_int, c_uint};

pub const AV_XVMC_ID: c_int = 0x1DC711C0;

#[derive(Debug)]
#[repr(C)]
pub struct xvmc_pix_fmt {
	pub xvmc_id: c_int,
	pub data_blocks: *mut c_short,
	pub mv_blocks: *mut c_void,
	pub allocated_mv_blocks: c_int,
	pub allocated_data_blocks: c_int,
	pub idct: c_int,
	pub unsigned_intra: c_int,
	pub p_surface: *mut c_void,
	pub p_past_surface: *mut c_void,
	pub p_future_surface: *mut c_void,
	pub picture_structure: c_uint,
	pub flags: c_uint,
	pub start_mv_blocks_num: c_int,
	pub filled_mv_blocks_num: c_int,
	pub next_free_data_block_num: c_int,
}
