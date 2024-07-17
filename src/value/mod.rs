pub mod error;
pub mod fraction;
pub mod number;

use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

pub use error::Error;
pub use error::Result;
pub use fraction::Fraction;
pub use number::Number;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(number::Number),
    Fraction(fraction::Fraction),
}

impl Value {
    pub fn simplify(self) -> Self {
        match self {
            Value::Number(number) => {
                if number.fract() == 0.0 {
                    Value::Number(number)
                } else {
                    Value::Fraction(Fraction::from(number))
                }
            }
            Value::Fraction(fraction) => {
                let fraction = fraction.simplify();
                if fraction.denominator == 1.0 {
                    Value::Number(Number::from(fraction))
                } else {
                    Value::Fraction(fraction)
                }
            }
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::Fraction(a), Value::Fraction(b)) => Value::Fraction(a + b),
            (Value::Number(a), Value::Fraction(b)) => Value::Fraction(a + b),
            (Value::Fraction(a), Value::Number(b)) => Value::Fraction(a + b),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            (Value::Fraction(a), Value::Fraction(b)) => Value::Fraction(a - b),
            (Value::Number(a), Value::Fraction(b)) => Value::Fraction(a - b),
            (Value::Fraction(a), Value::Number(b)) => Value::Fraction(a - b),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            (Value::Fraction(a), Value::Fraction(b)) => Value::Fraction(a * b),
            (Value::Number(a), Value::Fraction(b)) => Value::Fraction(a * b),
            (Value::Fraction(a), Value::Number(b)) => Value::Fraction(a * b),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            (Value::Fraction(a), Value::Fraction(b)) => Value::Fraction(a / b),
            (Value::Number(a), Value::Fraction(b)) => Value::Fraction(a / b),
            (Value::Fraction(a), Value::Number(b)) => Value::Fraction(a / b),
        }
    }
}
