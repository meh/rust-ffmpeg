use libc::{c_int, c_uint, c_double};

#[repr(C)]
pub struct AVLFG {
	pub state: [c_uint; 64],
	pub index: c_int,
}

#[inline(always)]
pub unsafe fn av_lfg_get(ctx: *mut AVLFG) -> c_uint {
	(*ctx).state[((*ctx).index & 63) as usize] = (*ctx).state[(((*ctx).index - 24) & 63) as usize] + (*ctx).state[(((*ctx).index - 55) & 63) as usize];
	(*ctx).index += 1;

	(*ctx).state[(((*ctx).index - 1) & 63) as usize]
}

#[inline(always)]
pub unsafe fn av_mlfg_get(ctx: *mut AVLFG) -> c_uint {
	let a = (*ctx).state[(((*ctx).index - 55) & 63) as usize];
	let b = (*ctx).state[(((*ctx).index - 24) & 63) as usize];
	let c = 2 * a * b + a + b;

	(*ctx).index += 1;
	(*ctx).state[(((*ctx).index - 1) & 63) as usize] = c;

	c
}

extern {
	pub fn av_lfg_init(c: *mut AVLFG, seed: c_uint);
	pub fn av_bmg_get(lfg: *mut AVLFG, out: *mut c_double);
}
