use libc::{c_int, int64_t};
use {AVRational, AV_TIME_BASE};

pub const AV_NOPTS_VALUE: int64_t = 0x8000000000000000u64 as int64_t;
pub const AV_TIME_BASE_Q: AVRational = AVRational {
    num: 1,
    den: AV_TIME_BASE as c_int,
};
