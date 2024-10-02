use num_complex::Complex32;
use std::{ffi::c_void, marker::PhantomData};

pub struct Resamp<O, H, S> {
    q: *mut c_void,
    rate: f32,
    drop_fn: fn(&mut Resamp<O, H, S>),
    _output_type: PhantomData<O>,
    _tap_type: PhantomData<H>,
    _sample_type: PhantomData<S>,
}

impl<O, H, S> Drop for Resamp<O, H, S> {
    fn drop(&mut self) {
        (self.drop_fn)(self);
    }
}

/// - output: Complex32
/// - taps: Complex32
/// - input: Complex32
pub type ResampCCC = Resamp<Complex32, Complex32, Complex32>;
/// - output: Complex32
/// - taps: f32
/// - input: Complex32
pub type ResampCRC = Resamp<Complex32, f32, Complex32>;
/// - output: f32
/// - taps: f32
/// - input: f32
pub type ResampRRR = Resamp<f32, f32, f32>;

macro_rules! impl_resamp(
    (
        mod_: $mod:ident,
        alias: $resamp_alias:ty,
        out: $O:ty,
        taps: $H:ty,
        input: $S:ty,
        cobj: $Q:path,
        complex_conv: $complex_conv:path,
        create_default_fn: $create_default_fn:path,
        create_fn: $create_fn:path,
        destroy_fn: $destroy_fn:path,
        exec_fn: $exec_fn:path
    ) => {
        mod $mod {
            use crate::{filt::resamp::Resamp};
            #[allow(unused_imports)]
            use ::num_complex::Complex32;
            use ::liquid_dsp_sys as sys;
            use ::std::{ffi::c_void, marker::PhantomData};

            impl Resamp<$O, $H, $S> {
                fn drop_fn(&mut self) {
                    unsafe {
                        let _ = $destroy_fn(self.q as $Q);
                    }
                }

                #[doc = concat!("See [resamp_", stringify!($mod), "_create_default](https://liquidsdr.org/api/resamp_", stringify!($mod), "/#create_default).")]
                pub fn create_default(rate: f32) -> Result<Resamp<$O, $H, $S>, String> {
                    let q = unsafe { $create_default_fn(rate) as *mut c_void };
                    if q.is_null() {
                        return Err("error".into());
                    }
                    let drop_fn = Self::drop_fn;
                    let _sample_type = PhantomData;
                    let _tap_type = PhantomData;
                    let _output_type = PhantomData;

                    Ok(Self {
                        q,
                        rate,
                        drop_fn,
                        _sample_type,
                        _tap_type,
                        _output_type,
                    })
                }

                #[doc = concat!("See [resamp_", stringify!($mod), "_create](https://liquidsdr.org/api/resamp_", stringify!($mod), "/#create).")]
                pub fn create(
                    rate: f32,
                    m: u32,
                    fc: f32,
                    as_: f32,
                    npfb: u32,
                ) -> Result<Resamp<$O, $H, $S>, String> {
                    let q = unsafe { $create_fn(rate, m, fc, as_, npfb) as *mut c_void };
                    if q.is_null() {
                        return Err("error".into());
                    }
                    let drop_fn = Self::drop_fn;
                    let _sample_type = PhantomData;
                    let _tap_type = PhantomData;
                    let _output_type = PhantomData;

                    Ok(Self {
                        q,
                        rate,
                        drop_fn,
                        _sample_type,
                        _tap_type,
                        _output_type,
                    })
                }

                /// Returns this filter's resample ratio.
                pub fn rate(&self) -> f32 {
                    self.rate
                }

                pub fn execute(&mut self, x: $S, y: &mut [$O]) -> Result<usize, String> {
                    let mut num_written: std::os::raw::c_uint = 0;
                    assert!(y.len() >= self.rate.ceil() as usize);
                    let x = $complex_conv(x);
                    let err = unsafe {
                        $exec_fn(
                            self.q as $Q,
                            x,
                            y.as_mut_ptr() as *mut _,
                            &mut num_written as *mut _,
                        )
                    };
                    if err != sys::liquid_error_code_LIQUID_OK as i32 {
                        Err("error".into())
                    } else {
                        Ok(num_written as usize)
                    }
                }
            }
        }
    }
);

impl_resamp!(
    mod_: cccf,
    alias: ResampCCC,
    out: Complex32,
    taps: Complex32,
    input: Complex32,
    cobj: sys::resamp_cccf,
    complex_conv: crate::complex::c,
    create_default_fn: sys::resamp_cccf_create_default,
    create_fn: sys::resamp_cccf_create,
    destroy_fn: sys::resamp_cccf_destroy,
    exec_fn: sys::resamp_cccf_execute
);

impl_resamp!(
    mod_: crcf,
    alias: ResampCRC,
    out: Complex32,
    taps: f32,
    input: Complex32,
    cobj: sys::resamp_crcf,
    complex_conv: crate::complex::c,
    create_default_fn: sys::resamp_crcf_create_default,
    create_fn: sys::resamp_crcf_create,
    destroy_fn: sys::resamp_crcf_destroy,
    exec_fn: sys::resamp_crcf_execute
);

impl_resamp!(
    mod_: rrrf,
    alias: ResampRRR,
    out: f32,
    taps: f32,
    input: f32,
    cobj: sys::resamp_rrrf,
    complex_conv: crate::complex::ident,
    create_default_fn: sys::resamp_rrrf_create_default,
    create_fn: sys::resamp_rrrf_create,
    destroy_fn: sys::resamp_rrrf_destroy,
    exec_fn: sys::resamp_rrrf_execute
);

#[cfg(test)]
mod tests {
    use super::{Complex32, ResampCCC, ResampCRC, ResampRRR};

    #[test]
    fn resamp_rrrf() {
        let rate = std::f32::consts::PI;
        let x: Vec<f32> = (0..100).map(|x| (x as f32).sin()).collect();
        let mut resamp = ResampRRR::create_default(rate).unwrap();
        let mut y: Vec<f32> = Vec::new();
        for &xx in x.iter() {
            let mut yy = [0.0; 4];
            let n = resamp.execute(xx, &mut yy).unwrap();
            y.extend(&yy[..n]);
        }
        assert_eq!(y.len(), 315);
    }

    #[test]
    fn resamp_crcf() {
        let rate = std::f32::consts::PI;
        let x: Vec<Complex32> = (0..100)
            .map(|x| (Complex32::from(x as f32)).sin())
            .collect();
        let mut resamp = ResampCRC::create_default(rate).unwrap();
        let mut y: Vec<Complex32> = Vec::new();
        for &xx in x.iter() {
            let mut yy = [Complex32::from(0.0); 4];
            let n = resamp.execute(xx, &mut yy).unwrap();
            y.extend(&yy[..n]);
        }
        assert_eq!(y.len(), 315);
    }

    #[test]
    fn resamp_cccf() {
        let rate = std::f32::consts::PI;
        let x: Vec<Complex32> = (0..100)
            .map(|x| (Complex32::from(x as f32)).sin())
            .collect();
        let mut resamp = ResampCCC::create_default(rate).unwrap();
        let mut y: Vec<Complex32> = Vec::new();
        for &xx in x.iter() {
            let mut yy = [Complex32::from(0.0); 4];
            let n = resamp.execute(xx, &mut yy).unwrap();
            y.extend(&yy[..n]);
        }
        assert_eq!(y.len(), 315);
    }
}
