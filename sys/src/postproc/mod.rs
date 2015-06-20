mod postproc;
pub use self::postproc::*;

#[cfg_attr(feature = "static", link(name = "postproc", kind = "static"))]
#[cfg_attr(not(feature = "static"), link(name = "postproc"))]
extern { }
