use libc::{c_char, c_int, c_uint};

pub const AV_CPU_FLAG_FORCE:    c_uint = 0x80000000;
pub const AV_CPU_FLAG_MMX:      c_uint = 0x0001;
pub const AV_CPU_FLAG_MMXEXT:   c_uint = 0x0002;
pub const AV_CPU_FLAG_MMX2:     c_uint = 0x0002;
pub const AV_CPU_FLAG_3DNOW:    c_uint = 0x0004;
pub const AV_CPU_FLAG_SSE:      c_uint = 0x0008;
pub const AV_CPU_FLAG_SSE2:     c_uint = 0x0010;
pub const AV_CPU_FLAG_SSE2SLOW: c_uint = 0x40000000;
pub const AV_CPU_FLAG_3DNOWEXT: c_uint = 0x0020;
pub const AV_CPU_FLAG_SSE3:     c_uint = 0x0040;
pub const AV_CPU_FLAG_SSE3SLOW: c_uint = 0x20000000;
pub const AV_CPU_FLAG_SSSE3:    c_uint = 0x0080;
pub const AV_CPU_FLAG_ATOM:     c_uint = 0x10000000;
pub const AV_CPU_FLAG_SSE4:     c_uint = 0x0100;
pub const AV_CPU_FLAG_SSE42:    c_uint = 0x0200;
pub const AV_CPU_FLAG_AVX:      c_uint = 0x4000;
pub const AV_CPU_FLAG_AVXSLOW:  c_uint = 0x8000000;
pub const AV_CPU_FLAG_XOP:      c_uint = 0x0400;
pub const AV_CPU_FLAG_FMA4:     c_uint = 0x0800;
pub const AV_CPU_FLAG_CMOV:     c_uint = 0x1001000;
pub const AV_CPU_FLAG_AVX2:     c_uint = 0x8000;
pub const AV_CPU_FLAG_FMA3:     c_uint = 0x10000;
pub const AV_CPU_FLAG_BMI1:     c_uint = 0x20000;
pub const AV_CPU_FLAG_BMI2:     c_uint = 0x40000;
pub const AV_CPU_FLAG_ALTIVEC:  c_uint = 0x0001;
pub const AV_CPU_FLAG_VSX:      c_uint = 0x0002;
pub const AV_CPU_FLAG_POWER8:   c_uint = 0x0004;
pub const AV_CPU_FLAG_ARMV5TE:  c_uint = 1 << 0;
pub const AV_CPU_FLAG_ARMV6:    c_uint = 1 << 1;
pub const AV_CPU_FLAG_ARMV6T2:  c_uint = 1 << 2;
pub const AV_CPU_FLAG_VFP:      c_uint = 1 << 3;
pub const AV_CPU_FLAG_VFPV3:    c_uint = 1 << 4;
pub const AV_CPU_FLAG_NEON:     c_uint = 1 << 5;
pub const AV_CPU_FLAG_ARMV8:    c_uint = 1 << 6;
pub const AV_CPU_FLAG_SETEND:   c_uint = 1 << 16;

extern {
	pub fn av_get_cpu_flags() -> c_int;
	pub fn av_force_cpu_flags(flags: c_int);
	pub fn av_parse_cpu_caps(flags: *mut c_uint, s: *const c_char) -> c_int;
	pub fn av_cpu_count() -> c_int;
}
