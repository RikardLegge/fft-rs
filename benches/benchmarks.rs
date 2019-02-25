use criterion::{Criterion, criterion_group, criterion_main};
use fft::transforms::{fft, dft};
use fft::sampler::sample;

fn gen_samples(pow: u32) -> Vec<f64> {
    return sample(&[10.0, 12.0], 2_usize.pow(pow));
}

fn benchmarks(c: &mut Criterion) {
    c.bench_function("dbf 64", move |b| {
        let samples = gen_samples(6);
        b.iter(|| {dft(&samples, 0..samples.len())})
    });

    c.bench_function("fft 64", move |b| {
        let samples = gen_samples(6);
        b.iter(|| {fft(&samples)})
    });

    c.bench_function("dbf 256", move |b| {
        let samples = gen_samples(8);
        b.iter(|| {dft(&samples, 0..samples.len())})
    });

    c.bench_function("fft 256", move |b| {
        let samples = gen_samples(8);
        b.iter(|| {fft(&samples)})
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);