use criterion::criterion_main;

mod filt {
    pub mod resamp {
        use criterion::{Criterion, Throughput};
        use liquid_dsp::filter::resamp::{
            // ResampCCC,
            ResampCRC,
            // ResampRRR
        };
        use num_complex::Complex32;

        pub fn resamp_ccc(c: &mut Criterion) {
            let rate = std::f32::consts::PI;
            let mut resamp = ResampCRC::create_default(rate).unwrap();
            let xs = vec![Complex32::from(1.0); 1024 * 1024];
            let mut yy = vec![Complex32::from(0.0); 1024 * 1024 * 4];

            let mut g = c.benchmark_group("ResampCCC single");
            g.throughput(Throughput::Elements(xs.len() as u64));
            g.bench_function("execute", |z| {
                z.iter(|| {
                    for x in xs.iter() {
                        resamp.execute(*x, &mut yy).unwrap();
                    }
                })
            });
            g.finish();

            let mut g = c.benchmark_group("ResampCCC block");
            g.throughput(Throughput::Elements(xs.len() as u64));
            g.bench_function("execute_batch", |z| {
                z.iter(|| {
                    resamp.execute_block(&xs, &mut yy).unwrap();
                })
            });
            g.finish();
        }

        pub fn resamp_crc(_c: &mut Criterion) {}
        pub fn resamp_rrr(_c: &mut Criterion) {}

        criterion::criterion_group!(resamp, resamp_crc, resamp_ccc, resamp_rrr);
    }
}

criterion_main!(filt::resamp::resamp);
