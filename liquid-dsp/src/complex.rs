use liquid_dsp_sys as sys;
use num_complex::Complex32;

#[inline(always)]
pub(crate) fn c(x: Complex32) -> sys::__BindgenComplex<f32> {
    let Complex32 { re, im } = x;
    sys::__BindgenComplex { re, im }
}

#[inline(always)]
pub(crate) fn ident<T>(x: T) -> T {
    x
}
