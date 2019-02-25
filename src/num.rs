use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::AddAssign;

#[derive(Copy, Clone, Debug)]
pub struct Complex(pub f64, pub f64);

impl Complex {
    pub fn len(&self) -> f64 {
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
