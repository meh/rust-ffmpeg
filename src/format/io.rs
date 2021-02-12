use crate::{
    ffi::*,
    format::context::{self, Context},
    Error, Format,
};
use libc::{c_int, c_void, SEEK_CUR, SEEK_END, SEEK_SET};
use std::{
    convert::TryInto,
    io::{self, Read, Seek, SeekFrom, Write},
    ptr, slice,
};

pub trait Stream: Read + Write + Seek {
    fn size(&self) -> io::Result<usize> {
        Ok(0)
    }
}

impl<T> Stream for T where T: Read + Write + Seek {}

#[derive(Debug)]
pub struct Io {
    ptr: *mut AVIOContext,
    proxy: *mut Box<dyn Stream>,
}

impl<F: Stream + Sized + 'static> From<F> for Io {
    fn from(value: F) -> Io {
        Io::new(Box::new(value))
    }
}

impl Io {
    pub unsafe fn as_ptr(&self) -> *const AVIOContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVIOContext {
        self.ptr
    }
}

#[no_mangle]
unsafe extern "C" fn read_packet(opaque: *mut c_void, buf: *mut u8, size: c_int) -> c_int {
    let mut proxy = Box::<Box<dyn Stream>>::from_raw(opaque.cast());
    let buffer = slice::from_raw_parts_mut(buf, size.try_into().unwrap());

    let result = match proxy.read(buffer) {
        Ok(0) => AVERROR_EOF,
        Ok(size) => size.try_into().unwrap(),
        Err(err) => err.raw_os_error().unwrap().try_into().unwrap(),
    };

    Box::into_raw(proxy);
    result.try_into().unwrap()
}

#[no_mangle]
unsafe extern "C" fn write_packet(opaque: *mut c_void, buf: *mut u8, size: c_int) -> c_int {
    let mut proxy = Box::<Box<dyn Stream>>::from_raw(opaque.cast());
    let buffer = slice::from_raw_parts(buf, size.try_into().unwrap());

    let result = match proxy.write(buffer) {
        Ok(size) => size,
        Err(err) => err.raw_os_error().unwrap().try_into().unwrap(),
    };

    Box::into_raw(proxy);
    result.try_into().unwrap()
}

#[no_mangle]
unsafe extern "C" fn seek(opaque: *mut c_void, offset: i64, whence: c_int) -> i64 {
    let mut proxy = Box::<Box<dyn Stream>>::from_raw(opaque.cast());
    let whence = match whence {
        AVSEEK_SIZE => {
            let result = match proxy.size() {
                Ok(size) => size,
                Err(err) => err.raw_os_error().unwrap().try_into().unwrap(),
            };
            Box::into_raw(proxy);
            return result.try_into().unwrap();
        }

        SEEK_SET => SeekFrom::Start(offset.try_into().unwrap()),
        SEEK_END => SeekFrom::End(offset.try_into().unwrap()),
        SEEK_CUR => SeekFrom::Current(offset.try_into().unwrap()),
        _ => unreachable!(),
    };

    let result = match proxy.seek(whence) {
        Ok(size) => size,
        Err(err) => err.raw_os_error().unwrap().try_into().unwrap(),
    };

    Box::into_raw(proxy);
    result.try_into().unwrap()
}

impl Io {
    pub fn new(proxy: Box<dyn Stream>) -> Self {
        Io::with_capacity(4096, proxy)
    }

    pub fn with_capacity(size: usize, proxy: Box<dyn Stream>) -> Self {
        unsafe {
            let proxy = Box::into_raw(Box::new(proxy));
            let ptr = avio_alloc_context(
                ptr::null_mut(),
                0,
                AVIO_FLAG_WRITE & AVIO_FLAG_DIRECT,
                proxy.cast(),
                Some(read_packet),
                Some(write_packet),
                Some(seek),
            );

            Io { proxy, ptr }
        }
    }
}

impl Drop for Io {
    fn drop(&mut self) {
        unsafe {
            avio_context_free(&mut self.ptr);
        }
    }
}

pub fn open<I: Into<Io>>(io: I, format: &Format) -> Result<Context, Error> {
    unsafe {
        let mut ps = avformat_alloc_context();
        let mut io = io.into();
        (*ps).pb = io.as_mut_ptr();

        match *format {
            Format::Input(ref format) => match avformat_open_input(
                &mut ps,
                ptr::null_mut(),
                format.as_ptr() as *mut _,
                ptr::null_mut(),
            ) {
                0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
                    r if r >= 0 => Ok(Context::Input(context::Input::wrap_with(ps, io))),
                    e => {
                        avformat_close_input(&mut ps);
                        Err(Error::from(e))
                    }
                },

                e => Err(Error::from(e)),
            },

            Format::Output(ref format) => match avformat_alloc_output_context2(
                &mut ps,
                format.as_ptr() as *mut _,
                ptr::null(),
                ptr::null(),
            ) {
                0 => Ok(Context::Output(context::Output::wrap_with(ps, io))),
                e => Err(Error::from(e)),
            },
        }
    }
}

pub fn input<I: Into<Io>>(io: I) -> Result<context::Input, Error> {
    unsafe {
        let mut ps = avformat_alloc_context();
        let mut io = io.into();
        (*ps).pb = io.as_mut_ptr();

        match avformat_open_input(&mut ps, ptr::null_mut(), ptr::null_mut(), ptr::null_mut()) {
            0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
                r if r >= 0 => Ok(context::Input::wrap_with(ps, io)),
                e => {
                    avformat_close_input(&mut ps);
                    Err(Error::from(e))
                }
            },

            e => Err(Error::from(e)),
        }
    }
}

pub fn output<I: Into<Io>>(io: I) -> Result<context::Output, Error> {
    unsafe {
        let mut ps = avformat_alloc_context();
        let mut io = io.into();
        (*ps).pb = io.as_mut_ptr();

        match avformat_alloc_output_context2(&mut ps, ptr::null_mut(), ptr::null(), ptr::null_mut())
        {
            0 => Ok(context::Output::wrap_with(ps, io)),
            e => Err(Error::from(e)),
        }
    }
}
