use libc::{c_void, c_int, c_float, c_double};

pub type FFTSample = c_float;

#[derive(Debug)]
#[repr(C)]
pub struct FFTComplex {
	re: FFTSample,
	im: FFTSample,
}

pub type FFTContext = c_void;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum RDFTransformType {
	DFT_R2C,
	IDFT_C2R,
	IDFT_R2C,
	DFT_C2R,
}

pub type RDFTContext = c_void;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum DCTTransformType {
	DCT_II = 0,
	DCT_III,
	DCT_I,
	DST_I,
}

pub type DCTContext = c_void;

extern {
	pub fn av_fft_init(nbits: c_int, inverse: c_int) -> *mut FFTContext;
	pub fn av_fft_permute(s: *mut FFTContext, z: *mut FFTComplex);
	pub fn av_fft_calc(s: *mut FFTContext, z: *mut FFTComplex);
	pub fn av_fft_end(s: *mut FFTContext);

	pub fn av_mdct_init(nbits: c_int, inverse: c_int, scale: c_double) -> *mut FFTContext;
	pub fn av_imdct_calc(s: *mut FFTContext, output: *mut FFTSample, input: *const FFTSample);
	pub fn av_imdct_half(s: *mut FFTContext, output: *mut FFTSample, input: *const FFTSample);
	pub fn av_mdct_calc(s: *mut FFTContext, output: *mut FFTSample, input: *const FFTSample);
	pub fn av_mdct_end(s: *mut FFTContext);

	pub fn av_rdft_init(nbits: c_int, trans: RDFTransformType) -> *mut RDFTContext;
	pub fn av_rdft_calc(s: *mut RDFTContext, data: *mut FFTSample);
	pub fn av_rdft_end(s: *mut RDFTContext);

	pub fn av_dct_init(nbits: c_int, kind: DCTTransformType) -> *mut DCTContext;
	pub fn av_dct_calc(s: *mut DCTContext, data: *mut FFTSample);
	pub fn av_dct_end(s: *mut DCTContext);
}
