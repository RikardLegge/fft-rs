use std::f64::consts::PI;

pub fn sample(frequencies: &[f64], sample_size: usize) -> Vec<f64> {
    let mut samples = Vec::with_capacity(sample_size);
    let sample_rate = 1.0 / sample_size as f64;

    for i in 0..sample_size {
        let mut sample = 0.0;
        for frequency in frequencies {
            let w = 2.0 * PI * frequency * (i as f64) * sample_rate;
            sample += w.sin();
        }
        samples.push(sample)
    }
    return samples;
}