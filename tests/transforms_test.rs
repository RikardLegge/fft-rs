
#[cfg(test)]
mod tests {
    use fft::transforms::{dft, fft};
    use fft::num::Complex;
    use fft::sampler::sample;

    #[test]
    fn dft_test() {
        let samples = sample(&[2.0, 4.0], 2_usize.pow(5));
        let f = dft(&samples, 0..5).iter().map(Complex::len).collect::<Vec<f64>>();
        assert!(f[0] < 1.0);
        assert!(f[1] < 1.0);
        assert!(f[2] > 1.0);
        assert!(f[3] < 1.0);
        assert!(f[4] > 1.0);
    }

    #[test]
    fn fft_test() {
        let samples = sample(&[2.0, 4.0], 2_usize.pow(5));
        let f = fft(&samples).iter().map(Complex::len).collect::<Vec<f64>>();

        assert!(f[0] < 1.0);
        assert!(f[1] < 1.0);
        assert!(f[2] > 1.0);
        assert!(f[3] < 1.0);
        assert!(f[4] > 1.0);
    }
}