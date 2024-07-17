use crate::value::Fraction;
use std::ops::{Add, Div, Mul, Sub};

pub type Number = f64;

impl From<Fraction> for Number {
    fn from(fraction: Fraction) -> Self {
        fraction.numerator / fraction.denominator
    }
}

impl Add<Fraction> for Number {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Self::Output {
        Fraction::from(self) + other
    }
}

impl Sub<Fraction> for Number {
    type Output = Fraction;

    fn sub(self, other: Fraction) -> Self::Output {
        Fraction::from(self) - other
    }
}

impl Mul<Fraction> for Number {
    type Output = Fraction;

    fn mul(self, other: Fraction) -> Self::Output {
        Fraction::from(self) * other
    }
}

impl Div<Fraction> for Number {
    type Output = Fraction;

    fn div(self, other: Fraction) -> Self::Output {
        Fraction::from(self) / other
    }
}
