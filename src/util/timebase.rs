use ffi::*;
use ::Rational;

pub const TIME_BASE: Rational = Rational(AV_TIME_BASE_Q.num, AV_TIME_BASE_Q.den);

pub trait Rescaling: Into<i64> + From<i64> {
	fn rescale<S, D>(self, source: S, destination: D) -> Self where S: Into<Rational>, D: Into<Rational> {
		unsafe {
			av_rescale_q(self.into(), source.into().into(), destination.into().into()).into()
		}
	}
}

impl Rescaling for i64 {}
