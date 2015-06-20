mod device;
pub use self::device::*;

#[cfg_attr(feature = "static", link(name = "avdevice", kind = "static"))]
#[cfg_attr(not(feature = "static"), link(name = "avdevice"))]
extern { }
