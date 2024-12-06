use criterion::criterion_main;

mod filt {
    pub mod resamp {
        use criterion::Criterion;
        // use liquid_dsp::filter::resamp::{ResampCCC, ResampCRC, ResampRRR};
        // use num_complex::Complex32;
        // use std::hint::black_box;

        pub fn resamp_ccc(c: &mut Criterion) {}
        pub fn resamp_CRC(c: &mut Criterion) {}
        pub fn resamp_rrr(c: &mut Criterion) {}

        criterion::criterion_group!(resamp, resamp_ccc, resamp_CRC, resamp_rrr);
    }
}

criterion_main!(filt::resamp::resamp);
