use AVRational;
use libc::{c_double, c_int, int64_t};

#[inline(always)]
pub unsafe fn av_make_q(num: c_int, den: c_int) -> AVRational {
    AVRational { num: num, den: den }
}

#[inline(always)]
pub unsafe fn av_cmp_q(a: AVRational, b: AVRational) -> c_int {
    let tmp: int64_t = a.num as int64_t * b.den as int64_t - b.num as int64_t * a.den as int64_t;

    if tmp != 0 {
        (((tmp ^ a.den as int64_t ^ b.den as int64_t) >> 63) | 1) as c_int
    } else if b.den != 0 && a.den != 0 {
        0
    } else if a.num != 0 && b.num != 0 {
        ((a.num as int64_t >> 31) - (b.num as int64_t >> 31)) as c_int
    } else {
        c_int::min_value()
    }
}

#[inline(always)]
pub unsafe fn av_q2d(a: AVRational) -> c_double {
    a.num as c_double / a.den as c_double
}

#[inline(always)]
pub unsafe fn av_inv_q(q: AVRational) -> AVRational {
    AVRational {
        num: q.den,
        den: q.num,
    }
}
