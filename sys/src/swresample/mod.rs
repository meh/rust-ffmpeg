mod resample;
pub use self::resample::*;
pub use self::resample::SwrDitherType::*;
pub use self::resample::SwrEngine::*;
pub use self::resample::SwrFilterType::*;

#[cfg_attr(feature = "static", link(name = "swresample", kind = "static"))]
#[cfg_attr(not(feature = "static"), link(name = "swresample"))]
extern { }
