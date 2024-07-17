use crate::value::{Result, Value};

pub enum Expression {
    Value(Value),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn evaluate(&self) -> Result<Value> {
        let value = match self {
            Expression::Value(value) => Ok(value.clone()),
            Expression::Add(lhs, rhs) => Ok(lhs.evaluate()? + rhs.evaluate()?),
            Expression::Subtract(lhs, rhs) => Ok(lhs.evaluate()? - rhs.evaluate()?),
            Expression::Multiply(lhs, rhs) => Ok(lhs.evaluate()? * rhs.evaluate()?),
            Expression::Divide(lhs, rhs) => Ok(lhs.evaluate()? / rhs.evaluate()?),
        }?;
        Ok(value.simplify())
    }
}
