mod resample;
pub use self::resample::*;
pub use self::resample::AVMixCoeffType::*;
pub use self::resample::AVResampleFilterType::*;
pub use self::resample::AVResampleDitherMethod::*;

#[cfg_attr(feature = "static", link(name = "avresample", kind = "static"))]
#[cfg_attr(not(feature = "static"), link(name = "avresample"))]
extern { }
