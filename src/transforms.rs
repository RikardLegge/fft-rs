use std::f64::consts::PI;
use std::ops::Range;
use crate::num::Complex;

pub fn fft(samples: &[f64]) -> Vec<Complex> {
    let n = samples.len();
    let mut frequencies = vec![Complex(0.0, 0.0); n];
    if n == 0 {
        return frequencies;
    }
    fft_divide_and_conquer(samples, n, 1, &mut frequencies);
    return frequencies;
}

// https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm#Pseudocode
fn fft_divide_and_conquer(samples: &[f64], n: usize, s: usize, frequencies: &mut [Complex]) {
    if n == 1 {
        frequencies[0] = Complex(samples[0], 0.0);
        return;
    }
    fft_divide_and_conquer(&samples[0..], n / 2, 2 * s, &mut frequencies[0..n / 2]);
    fft_divide_and_conquer(&samples[s..], n / 2, 2 * s, &mut frequencies[n / 2..]);

    for k in 0..n / 2 {
        let angle = 2.0 * PI * k as f64 / n as f64;
        let w = Complex(angle.cos(), -angle.sin());

        let t = frequencies[k];
        let u = w * frequencies[k + n / 2];
        frequencies[k] = t + u;
        frequencies[k + n / 2] = t - u;
    }
}

// https://en.wikipedia.org/wiki/Discrete_Fourier_transform#Definition
pub fn dft(samples: &[f64], frequency_range: Range<usize>) -> Vec<Complex> {
    let mut freqnencies = Vec::new();
    for k in frequency_range {
        let mut frequency = Complex(0.0, 0.0);
        for (n, &sample) in samples.iter().enumerate() {
            let angle = 2.0 * PI * (k * n) as f64 / samples.len() as f64;
            let w = Complex(angle.cos(), -angle.sin());
            frequency += Complex(sample, 0.0) * w;
        }
        freqnencies.push(frequency);
    }
    return freqnencies;
}
