mod scale;
pub use self::scale::*;

#[cfg_attr(feature = "static", link(name = "swscale", kind = "static"))]
#[cfg_attr(not(feature = "static"), link(name = "swscale"))]
extern { }
