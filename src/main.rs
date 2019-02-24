use std::ops::Range;
use std::f64::consts::PI;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::AddAssign;

#[derive(Copy, Clone, Debug)]
struct Complex(f64, f64);

impl Complex {
    fn len(&self) -> f64 {
        let re = self.0;
        let im = self.1;
        (re.powi(2) + im.powi(2)).sqrt()
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Complex(self.0 * rhs.0 - self.1 * rhs.1, self.0 * rhs.1 + self.1 * rhs.0)
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Complex) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        Complex(self.0 - rhs.0, self.1 - rhs.1)
    }
}

fn main() {
    let samples = sample(&[10.0, 12.0], 128);
    dbg!(dft(&samples, 5..15).iter().map(|im| { im.len() }).collect::<Vec<f64>>());
    dbg!(fft(&samples).iter().map(|im| { im.len() }).collect::<Vec<f64>>());
}

fn fft(samples: &[f64]) -> Vec<Complex> {
    let n = samples.len();
    if n == 0 {
        return Vec::new();
    }
    return fft_internal(samples, n, 1);
}

fn fft_internal(samples: &[f64], n: usize, s: usize) -> Vec<Complex> {
    if n == 1 {
        return vec![Complex(samples[0], 0.0)];
    }
    let mut frequencies = fft_internal(&samples[0..], n / 2, 2 * s);
    let frequencies_n = fft_internal(&samples[s..], n / 2, 2 * s);
    frequencies.extend_from_slice(&frequencies_n);

    for k in 0..n / 2 {
        let angle = 2.0 * PI * k as f64 / n as f64;
        let w = Complex(angle.cos(), -angle.sin());

        let t = frequencies[k];
        let u = w * frequencies[k + n / 2];
        frequencies[k] = t + u;
        frequencies[k + n / 2] = t - u;
    }

    return frequencies;
}

// https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm#Data_reordering,_bit_reversal,_and_in-place_algorithms
fn fft_2(samples: &[f64], frequency_range: Range<usize>) -> Vec<Complex> {
    let mut frequencies = vec![Complex(0.0, 0.0); samples.len()];
    let n = samples.len();
    if n == 0 {
        return frequencies;
    }
    if n == 1 {
        if frequency_range.start == 0 {
            frequencies[0] = Complex(samples[0], 0.0);
        }
        return frequencies;
    }

    // bit-reverse-copy
    for k in 0..n {
        frequencies[n - 1 - k] = Complex(samples[k], 0.0);
    }

    let log_n = (n as f64).log2() as u32;
    for s in 1..=log_n {
        let m = 2_usize.pow(s) as usize;
        let angle = 2.0 * PI / m as f64;
        let wm = Complex(angle.cos(), -angle.sin());

        for k in (0..n).step_by(m) {
            let mut w = Complex(1.0, 0.0);
            for j in 0..m / 2 {
                let t = w * frequencies[k + j + m / 2];
                let u = frequencies[k + j];
                frequencies[k + j] = u + t;
                frequencies[k + j + m / 2] = u - t;
                w = w * wm;
            }
        }
    }
    return frequencies;
}

// https://en.wikipedia.org/wiki/Discrete_Fourier_transform#Definition
fn dft(samples: &[f64], frequency_range: Range<usize>) -> Vec<Complex> {
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

fn sample(frequencies: &[f64], sample_size: usize) -> Vec<f64> {
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
