use std::{panic, process};

use crate::ffi::*;
use libc::{c_int, c_void};

pub struct Interrupt {
    pub interrupt: AVIOInterruptCB,
}

extern "C" fn callback<F>(opaque: *mut c_void) -> c_int
where
    F: FnMut() -> bool,
{
    match panic::catch_unwind(|| (unsafe { &mut *(opaque as *mut F) })()) {
        Ok(ret) => ret as c_int,
        Err(_) => process::abort(),
    }
}

pub fn new<F>(opaque: Box<F>) -> Interrupt
where
    F: FnMut() -> bool,
{
    let interrupt_cb = AVIOInterruptCB {
        callback: Some(callback::<F>),
        opaque: Box::into_raw(opaque) as *mut c_void,
    };
    Interrupt {
        interrupt: interrupt_cb,
    }
}
