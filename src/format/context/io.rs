use ffi::*;
use libc::c_int;
use libc::c_void;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::slice;

/// A safe wrapper around AVIOContext with a writer and seeker.
pub struct IOContextWrite<O>
where
    O: Write + Seek,
{
    buffer: *mut c_void,
    inner: *mut AVIOContext,
    output: Box<O>,
}

unsafe extern "C" fn write_packet<O: Write + Seek>(
    opaque: *mut c_void,
    buf: *mut u8,
    buf_size: c_int,
) -> c_int {
    let unwrapped = opaque as *mut O;

    let slice = slice::from_raw_parts(buf, buf_size as usize);
    let output = &mut unwrapped.as_mut().unwrap();

    if let Err(..) = output.write_all(slice) {
        return -1;
    }

    0
}

unsafe extern "C" fn seek<O: Write + Seek>(opaque: *mut c_void, offset: i64, whence: c_int) -> i64 {
    let unwrapped = opaque as *mut O;
    let output = &mut unwrapped.as_mut().unwrap();

    let from = match whence {
        SEEK_SET => SeekFrom::Start(offset as u64),
        SEEK_CUR => SeekFrom::Current(offset),
        SEEK_END => SeekFrom::End(offset),
        AVSEEK_SIZE | _ => return -1,
    };

    return match output.seek(from) {
        Err(..) => -1,
        Ok(final_offset) => final_offset as i64,
    };
}

impl<O> IOContextWrite<O>
where
    O: Write + Seek,
{
    pub fn new(output: O, buffer_size: i32) -> IOContextWrite<O> {
        let mut boxed_output = Box::new(output);
        let boxed_output_ptr: *mut O = &mut *boxed_output;

        // Allocate Buffer
        let buffer = unsafe { av_malloc(buffer_size as usize) };

        let inner = unsafe {
            avio_alloc_context(
                buffer as *mut u8,
                buffer_size,
                1,
                boxed_output_ptr as *mut c_void,
                None,
                Some(write_packet::<O>),
                Some(seek::<O>),
            )
        };

        IOContextWrite {
            buffer,
            output: boxed_output,
            inner,
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVIOContext {
        self.inner
    }

    pub fn inner(&self) -> &O {
        &*self.output
    }
}

impl<O> Drop for IOContextWrite<O>
where
    O: Write + Seek,
{
    fn drop(&mut self) {
        unsafe {
            av_free(self.inner as *mut c_void);
            av_free(self.buffer);
        };
    }
}
