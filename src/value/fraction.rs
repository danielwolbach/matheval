use crate::value::{Error, Number, Result};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Fraction {
    pub numerator: Number,
    pub denominator: Number,
}

impl Fraction {
    pub fn new(numerator: Number, denominator: Number) -> Result<Self> {
        if denominator == 0.0 {
            Err(Error::DivideByZero)
        } else {
            Ok(Self {
                numerator,
                denominator,
            }
            .simplify())
        }
    }

    pub fn simplify(self) -> Self {
        let gcd = Self::gcd(self.numerator, self.denominator);
        Self {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }

    pub fn inverse(self) -> Result<Self> {
        Self::new(self.denominator, self.numerator)
    }

    fn gcd(a: Number, b: Number) -> Number {
        // TODO Move this function to where it makes sense.
        let mut a = a.abs();
        let mut b = b.abs();
        while b != 0.0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}

impl From<Number> for Fraction {
    fn from(number: Number) -> Self {
        let mut numerator = number;
        let mut denominator = 1.0;
        while numerator.fract() != 0.0 {
            numerator *= 10.0;
            denominator *= 10.0;
        }
        Self::new(numerator, denominator).expect("Denominator is zero.")
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        let this = self.clone().simplify();
        let other = other.clone().simplify();
        this.numerator == other.numerator && this.denominator == other.denominator
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let numerator = self.numerator * other.denominator + other.numerator * self.denominator;
        let denominator = self.denominator * other.denominator;
        Self {
            numerator,
            denominator,
        }
    }
}

impl Add<Number> for Fraction {
    type Output = Self;

    fn add(self, other: Number) -> Self::Output {
        self + Fraction::from(other)
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let numerator = self.numerator * other.denominator - other.numerator * self.denominator;
        let denominator = self.denominator * other.denominator;
        Self {
            numerator,
            denominator,
        }
    }
}

impl Sub<Number> for Fraction {
    type Output = Self;

    fn sub(self, other: Number) -> Self::Output {
        self - Fraction::from(other)
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let numerator = self.numerator * other.numerator;
        let denominator = self.denominator * other.denominator;
        Self {
            numerator,
            denominator,
        }
    }
}

impl Mul<Number> for Fraction {
    type Output = Self;

    fn mul(self, other: Number) -> Self::Output {
        self * Fraction::from(other)
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let numerator = self.numerator * other.denominator;
        let denominator = self.denominator * other.numerator;
        Self {
            numerator,
            denominator,
        }
    }
}

impl Div<Number> for Fraction {
    type Output = Self;

    fn div(self, other: Number) -> Self::Output {
        self / Fraction::from(other)
    }
}
