use mathl::value::Fraction;
use mathl::Expression;
use mathl::Value;

#[test]
fn add_numbers_1() {
    let expression = Expression::Add(
        Box::new(Expression::Value(Value::Number(1.0))),
        Box::new(Expression::Value(Value::Number(2.0))),
    );

    assert_eq!(expression.evaluate().unwrap(), Value::Number(3.0));
}

#[test]
fn add_numbers_2() {
    let expression = Expression::Add(
        Box::new(Expression::Value(Value::Number(-5.0))),
        Box::new(Expression::Value(Value::Number(3.5))),
    );

    assert_eq!(
        expression.evaluate().unwrap(),
        Value::Fraction(Fraction::from(-1.5))
    );
}

#[test]
fn add_numbers_3() {
    let expression = Expression::Subtract(
        Box::new(Expression::Value(Value::Number(10.0))),
        Box::new(Expression::Value(Value::Number(4.5))),
    );

    assert_eq!(
        expression.evaluate().unwrap(),
        Value::Fraction(Fraction::from(5.5))
    );
}

#[test]
fn add_numbers_4() {
    let expression = Expression::Multiply(
        Box::new(Expression::Value(Value::Number(2.0))),
        Box::new(Expression::Value(Value::Number(3.5))),
    );

    assert_eq!(expression.evaluate().unwrap(), Value::Number(7.0));
}

#[test]
fn add_numbers_5() {
    let expression = Expression::Divide(
        Box::new(Expression::Value(Value::Number(10.0))),
        Box::new(Expression::Value(Value::Number(2.0))),
    );

    assert_eq!(expression.evaluate().unwrap(), Value::Number(5.0));
}

#[test]
fn add_numbers_6() {
    let expression = Expression::Subtract(
        Box::new(Expression::Add(
            Box::new(Expression::Value(Value::Number(10.0))),
            Box::new(Expression::Value(Value::Number(5.0))),
        )),
        Box::new(Expression::Value(Value::Number(3.0))),
    );

    assert_eq!(expression.evaluate().unwrap(), Value::Number(12.0));
}
