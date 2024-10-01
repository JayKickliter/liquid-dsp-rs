use liquid_dsp_bindings_sys as sys;
use std::{ffi::c_void, marker::PhantomData};

pub struct Resamp<Ts, To> {
    cobj: *const c_void,
    rate: f32,
    _sample_type: PhantomData<Ts>,
    _output_type: PhantomData<To>,
}

impl Resamp<f32, f32> {
    pub fn create_default(rate: f32) -> Result<Resamp<f32, f32>, String> {
        let cobj = unsafe { sys::resamp_rrrf_create_default(rate) as *const c_void };
        if cobj.is_null() {
            return Err("error".into());
        }
        let _sample_type = PhantomData;
        let _output_type = PhantomData;

        Ok(Self {
            cobj,
            rate,
            _sample_type,
            _output_type,
        })
    }

    pub fn create(
        rate: f32,
        m: u32,
        fc: f32,
        as_: f32,
        npfb: u32,
    ) -> Result<Resamp<f32, f32>, String> {
        let cobj = unsafe { sys::resamp_rrrf_create(rate, m, fc, as_, npfb) as *const c_void };
        if cobj.is_null() {
            return Err("error".into());
        }
        let _sample_type = PhantomData;
        let _output_type = PhantomData;

        Ok(Self {
            cobj,
            rate,
            _sample_type,
            _output_type,
        })
    }

    pub fn execute(&mut self, x: f32, y: &mut [f32]) -> Result<usize, String> {
        let mut num_written: std::os::raw::c_uint = 0;
        assert!(y.len() >= self.rate.ceil() as usize);
        let err = unsafe {
            sys::resamp_rrrf_execute(
                self.cobj as sys::resamp_rrrf,
                x,
                y.as_mut_ptr(),
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

#[cfg(test)]
mod tests {
    use super::Resamp;

    #[test]
    fn resamp_create() {
        let rate = std::f32::consts::PI;
        let x: Vec<f32> = (0..100).map(|x| (x as f32).sin()).collect();
        let mut resamp: Resamp<f32, f32> = Resamp::create_default(rate).unwrap();
        let mut y: Vec<f32> = Vec::new();
        for &xx in x.iter() {
            let mut yy = [0.0; 4];
            let n = resamp.execute(xx, &mut yy).unwrap();
            y.extend(&yy[..n]);
        }
    }
}
