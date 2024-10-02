use crate::complex::c;
use liquid_dsp_sys as sys;
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

/// - input: Complex32
/// - taps: Complex32
/// - output: Complex32
pub type ResampCCC = Resamp<Complex32, Complex32, Complex32>;
/// - input: Complex32
/// - taps: f32
/// - output: Complex32
pub type ResampCRC = Resamp<Complex32, f32, Complex32>;
/// - input: Complex32
/// - taps: f32
/// - output: Complex32
pub type ResampRRR = Resamp<f32, f32, f32>;

mod ccc {
    use super::*;
    type O = Complex32;
    type H = Complex32;
    type S = Complex32;
    type Q = sys::resamp_cccf;

    impl Resamp<O, H, S> {
        fn drop_fn(&mut self) {
            unsafe {
                let _ = sys::resamp_cccf_destroy(self.q as Q);
            }
        }

        /// See [resamp_cfcf_create_default](https://liquidsdr.org/api/resamp_crcf/#create_default).
        pub fn create_default(rate: f32) -> Result<Resamp<O, H, S>, String> {
            let q = unsafe { sys::resamp_cccf_create_default(rate) as *mut c_void };
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

        /// See [resamp_crcf_create](https://liquidsdr.org/api/resamp_crcf/#create).
        pub fn create(
            rate: f32,
            m: u32,
            fc: f32,
            as_: f32,
            npfb: u32,
        ) -> Result<Resamp<O, H, S>, String> {
            let q = unsafe { sys::resamp_cccf_create(rate, m, fc, as_, npfb) as *mut c_void };
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

        pub fn execute(&mut self, x: S, y: &mut [O]) -> Result<usize, String> {
            let mut num_written: std::os::raw::c_uint = 0;
            assert!(y.len() >= self.rate.ceil() as usize);
            let err = unsafe {
                sys::resamp_cccf_execute(
                    self.q as Q,
                    c(x),
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

mod crc {
    use super::*;
    type O = Complex32;
    type H = f32;
    type S = Complex32;
    type Q = sys::resamp_crcf;

    impl Resamp<O, H, S> {
        fn drop_fn(&mut self) {
            unsafe {
                let _ = sys::resamp_crcf_destroy(self.q as Q);
            }
        }

        /// See [resamp_cfcf_create_default](https://liquidsdr.org/api/resamp_cfcf/#create_default).
        pub fn create_default(rate: f32) -> Result<Resamp<O, H, S>, String> {
            let q = unsafe { sys::resamp_crcf_create_default(rate) as *mut c_void };
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

        /// See [resamp_crcf_create](https://liquidsdr.org/api/resamp_crcf/#create).
        pub fn create(
            rate: f32,
            m: u32,
            fc: f32,
            as_: f32,
            npfb: u32,
        ) -> Result<Resamp<O, H, S>, String> {
            let q = unsafe { sys::resamp_crcf_create(rate, m, fc, as_, npfb) as *mut c_void };
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
        ///
        /// See [resamp_crcf_get_rate](https://liquidsdr.org/api/resamp_crcf/#get_rate).
        pub fn rate(&self) -> f32 {
            self.rate
        }

        pub fn execute(&mut self, x: S, y: &mut [O]) -> Result<usize, String> {
            let mut num_written: std::os::raw::c_uint = 0;
            assert!(y.len() >= self.rate.ceil() as usize);
            let err = unsafe {
                sys::resamp_crcf_execute(
                    self.q as Q,
                    c(x),
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

mod rrr {
    use super::*;
    type O = f32;
    type S = f32;
    type H = f32;
    type Q = sys::resamp_rrrf;

    impl Resamp<O, H, S> {
        fn drop_fn(&mut self) {
            unsafe {
                let _ = sys::resamp_rrrf_destroy(self.q as Q);
            }
        }

        /// See [resamp_rrrf_create_default](https://liquidsdr.org/api/resamp_rrrf/#create_default).
        pub fn create_default(rate: f32) -> Result<Resamp<O, H, S>, String> {
            let q = unsafe { sys::resamp_rrrf_create_default(rate) as *mut c_void };
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

        /// See [resamp_rrrf_create](https://liquidsdr.org/api/resamp_rrrf/#create).
        pub fn create(
            rate: f32,
            m: u32,
            fc: f32,
            as_: f32,
            npfb: u32,
        ) -> Result<Resamp<O, H, S>, String> {
            let q = unsafe { sys::resamp_rrrf_create(rate, m, fc, as_, npfb) as *mut c_void };
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

        pub fn execute(&mut self, x: S, y: &mut [O]) -> Result<usize, String> {
            let mut num_written: std::os::raw::c_uint = 0;
            assert!(y.len() >= self.rate.ceil() as usize);
            let err = unsafe {
                sys::resamp_rrrf_execute(self.q as Q, x, y.as_mut_ptr(), &mut num_written as *mut _)
            };
            if err != sys::liquid_error_code_LIQUID_OK as i32 {
                Err("error".into())
            } else {
                Ok(num_written as usize)
            }
        }
    }
}

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
        dbg!(y);
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
        dbg!(y);
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
        dbg!(y);
    }
}
