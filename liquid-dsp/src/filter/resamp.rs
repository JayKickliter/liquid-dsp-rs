use num_complex::Complex32;
use std::{ffi::c_void, marker::PhantomData};

pub struct Resamp<O, H, S> {
    q: *mut c_void,
    rate: f32,
    clone_fn: fn(&Resamp<O, H, S>) -> Self,
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

impl<O, H, S> Clone for Resamp<O, H, S> {
    fn clone(&self) -> Self {
        (self.clone_fn)(self)
    }
}

macro_rules! impl_resamp(
    (
        mod_: $mod:ident,
        alias: $resamp_alias:ident,
        out: $O:ty,
        taps: $H:ty,
        input: $S:ty,
        cobj: $Q:path,
        complex_conv: $complex_conv:path,
        copy_fn: $copy_fn:path,
        create_default_fn: $create_default_fn:path,
        create_fn: $create_fn:path,
        destroy_fn: $destroy_fn:path,
        exec_fn: $exec_fn:path,
        exec_block_fn: $exec_block_fn:path
    ) => {
        #[doc = concat!("- output: ", stringify!($O))]
        #[doc = concat!("- taps: ", stringify!($H))]
        #[doc = concat!("- input: ", stringify!($S))]
        pub type $resamp_alias = Resamp<$O, $H, $S>;

        mod $mod {
            use crate::{Error, ErrorKind, error::PassThrough, filter::resamp::Resamp};
            #[allow(unused_imports)]
            use ::num_complex::Complex32;
            use ::liquid_dsp_sys as sys;
            use ::std::{ffi::{c_void, c_uint}, marker::PhantomData, convert::TryFrom};

            impl Resamp<$O, $H, $S> {
                fn drop_fn(&mut self) {
                    unsafe {
                        let _ = $destroy_fn(self.q as $Q);
                    }
                }

                fn clone_fn(&self) -> Self {
                    let q = unsafe {$copy_fn(self.q as $Q) as *mut c_void};
                    Self {
                        q,
                        rate: self.rate,
                        drop_fn: self.drop_fn,
                        clone_fn: self.clone_fn,
                        _output_type: PhantomData,
                        _tap_type: PhantomData,
                        _sample_type: PhantomData,
                    }
                }

                /// Create arbitrary resampler object with a specified
                /// input resampling rate and default parameters. This
                /// is a simplified method to provide a basic
                /// resampler with a baseline set of parameters,
                /// abstracting away some of the complexities with the
                /// filterbank design. The default parameters are
                ///
                /// - `m`: 7 (filter semi-length)
                /// - `fc`: `min(0.49, _rate/2)` (filter cutoff frequency)
                /// - `sa`: 60 dB (filter stop-band attenuation)
                /// - `npfb`: 64 (number of filters in the bank)
                ///
                #[doc = concat!("See [resamp_", stringify!($mod), "_create_default](https://liquidsdr.org/api/resamp_", stringify!($mod), "/#create_default).")]
                pub fn create_default(rate: f32) -> Result<Self, Error> {
                    let q = unsafe { $create_default_fn(rate) as *mut c_void };
                    if q.is_null() {
                        return Err(ErrorKind::Input.err_with_ctx(concat!("resamp_", stringify!($mod), "create_default returned NULL")));
                    }
                    let drop_fn = Self::drop_fn;
                    let clone_fn = Self::clone_fn;
                    let _sample_type = PhantomData;
                    let _tap_type = PhantomData;
                    let _output_type = PhantomData;

                    Ok(Self {
                        q,
                        rate,
                        drop_fn,
                        clone_fn,
                        _sample_type,
                        _tap_type,
                        _output_type,
                    })
                }

                /// Create arbitrary resampler object from filter
                /// prototype.
                ///
                /// - `rate`: arbitrary resampling rate, `0 < rate`
                /// - `m`: filter semi-length (delay), `0 < m`
                /// - `fc`: filter cutoff frequency, `0 < fc < 0.5`
                /// - `sa`: filter stop-band attenuation (dB), `0 < sa`
                /// - `npfb`: number of filters in the bank, `0 < npfb`
                ///
                #[doc = concat!("See [resamp_", stringify!($mod), "_create](https://liquidsdr.org/api/resamp_", stringify!($mod), "/#create).")]
                pub fn create(
                    rate: f32,
                    m: u32,
                    fc: f32,
                    sa: f32,
                    npfb: u32,
                ) -> Result<Self, ErrorKind> {
                    let q = unsafe { $create_fn(rate, m, fc, sa, npfb) as *mut c_void };
                    if q.is_null() {
                        return Err(ErrorKind::Input);
                    }
                    let clone_fn = Self::clone_fn;
                    let drop_fn = Self::drop_fn;
                    let _sample_type = PhantomData;
                    let _tap_type = PhantomData;
                    let _output_type = PhantomData;

                    Ok(Self {
                        q,
                        rate,
                        clone_fn,
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

                /// Execute arbitrary resampler on a single input
                /// sample and store the resulting samples in the
                /// output array. The number of output samples depends
                /// upon the resampling rate but will be at most ⌈r⌉
                /// samples.
                ///
                /// - `x`: single input sample
                /// - `y`: output sample slice
                ///
                /// Returns the number of output samples written to `y`, or an error.
                ///
                #[doc = concat!("See [resamp_", stringify!($mod), "_execute](https://liquidsdr.org/api/resamp_", stringify!($mod), "/#execute).")]
                pub fn execute(&mut self, x: $S, y: &mut [$O]) -> Result<usize, ErrorKind> {
                    if y.len() < self.rate.ceil() as usize {
                        return Err(ErrorKind::Range)
                    }

                    let mut num_written: c_uint = 0;
                    let x = $complex_conv(x);
                    let err = unsafe {
                        $exec_fn(
                            self.q as $Q,
                            x,
                            y.as_mut_ptr() as *mut _,
                            &mut num_written as *mut _,
                        )
                    };
                    let _ = PassThrough::try_from(err)?;
                    Ok(num_written as usize)
                }

                /// Execute arbitrary resampler on a block of input samples and store
                /// the resulting samples in the output array. The number of output
                /// samples depends upon the resampling rate and the number of input
                /// samples but will be at most ⌈rnx⌉ samples.
                ///
                /// - x: input slice
                /// - y: output sample slice
                ///
                /// Returns the number of output samples written to `y`, or an error.
                ///
                #[doc = concat!("See [resamp_", stringify!($mod), "_execute_block](https://liquidsdr.org/api/resamp_", stringify!($mod), "/#execute_block).")]
                pub fn execute_block(&mut self, x: &[$S], y: &mut [$O]) -> Result<usize, ErrorKind> {
                    if y.len() < (self.rate * x.len() as c_uint as f32).ceil() as usize {
                        return Err(ErrorKind::Range)
                    }

                    let mut num_written: c_uint = 0;
                    let err = unsafe {
                        $exec_block_fn(
                            self.q as $Q,
                            x.as_ptr() as *mut _,
                            x.len() as c_uint,
                            y.as_mut_ptr() as *mut _,
                            &mut num_written as *mut _,
                        )
                    };
                    let _ = PassThrough::try_from(err)?;
                    Ok(num_written as usize)
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
    copy_fn: sys::resamp_cccf_copy,
    create_default_fn: sys::resamp_cccf_create_default,
    create_fn: sys::resamp_cccf_create,
    destroy_fn: sys::resamp_cccf_destroy,
    exec_fn: sys::resamp_cccf_execute,
    exec_block_fn: sys::resamp_cccf_execute_block
);

impl_resamp!(
    mod_: crcf,
    alias: ResampCRC,
    out: Complex32,
    taps: f32,
    input: Complex32,
    cobj: sys::resamp_crcf,
    complex_conv: crate::complex::c,
    copy_fn: sys::resamp_crcf_copy,
    create_default_fn: sys::resamp_crcf_create_default,
    create_fn: sys::resamp_crcf_create,
    destroy_fn: sys::resamp_crcf_destroy,
    exec_fn: sys::resamp_crcf_execute,
    exec_block_fn: sys::resamp_crcf_execute_block
);

impl_resamp!(
    mod_: rrrf,
    alias: ResampRRR,
    out: f32,
    taps: f32,
    input: f32,
    cobj: sys::resamp_rrrf,
    complex_conv: crate::complex::ident,
    copy_fn: sys::resamp_rrrf_copy,
    create_default_fn: sys::resamp_rrrf_create_default,
    create_fn: sys::resamp_rrrf_create,
    destroy_fn: sys::resamp_rrrf_destroy,
    exec_fn: sys::resamp_rrrf_execute,
    exec_block_fn: sys::resamp_rrrf_execute_block
);

#[cfg(test)]
mod tests {
    use super::{Complex32, ResampCCC, ResampCRC, ResampRRR};

    #[test]
    fn resamp_rrrf() {
        let rate = std::f32::consts::PI;
        let x: Vec<f32> = (0..101).map(|x| (x as f32).sin()).collect();
        let mut resamp = ResampRRR::create_default(rate).unwrap();
        let mut y: Vec<f32> = Vec::new();
        for &xx in x[0..100].iter() {
            let mut yy = [0.0; 4];
            let n = resamp.execute(xx, &mut yy).unwrap();
            y.extend(&yy[..n]);
        }
        assert_eq!(y.len(), 315);
        let mut resamp_clone = resamp.clone();
        let mut y = [0.0; 4];
        let mut y_clone = [0.0; 4];
        resamp.execute(x[100], &mut y).unwrap();
        resamp_clone.execute(x[100], &mut y_clone).unwrap();
        assert_eq!(y, y_clone);
    }

    #[test]
    fn resamp_crcf() {
        let rate = std::f32::consts::PI;
        let x: Vec<Complex32> = (0..101)
            .map(|x| (Complex32::from(x as f32)).sin())
            .collect();
        let mut resamp = ResampCRC::create_default(rate).unwrap();
        let mut y: Vec<Complex32> = Vec::new();
        for &xx in x[0..100].iter() {
            let mut yy = [Complex32::from(0.0); 4];
            let n = resamp.execute(xx, &mut yy).unwrap();
            y.extend(&yy[..n]);
        }
        assert_eq!(y.len(), 315);
        let mut resamp_clone = resamp.clone();
        let mut y = [Complex32::from(0.0); 4];
        let mut y_clone = [Complex32::from(0.0); 4];
        resamp.execute(x[100], &mut y).unwrap();
        resamp_clone.execute(x[100], &mut y_clone).unwrap();
        assert_eq!(y, y_clone);
    }

    #[test]
    fn resamp_cccf() {
        let rate = std::f32::consts::PI;
        let x: Vec<Complex32> = (0..101)
            .map(|x| (Complex32::from(x as f32)).sin())
            .collect();
        let mut resamp = ResampCCC::create_default(rate).unwrap();
        let mut y: Vec<Complex32> = Vec::new();
        for &xx in x[0..100].iter() {
            let mut yy = [Complex32::from(0.0); 4];
            let n = resamp.execute(xx, &mut yy).unwrap();
            y.extend(&yy[..n]);
        }
        assert_eq!(y.len(), 315);
        let mut resamp_clone = resamp.clone();
        let mut y = [Complex32::from(0.0); 4];
        let mut y_clone = [Complex32::from(0.0); 4];
        resamp.execute(x[100], &mut y).unwrap();
        resamp_clone.execute(x[100], &mut y_clone).unwrap();
        assert_eq!(y, y_clone);
    }
}
