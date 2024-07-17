use matheval::value::Fraction;
use matheval::Expression;
use matheval::Value;

fn main() {
    let expression = Expression::Multiply(
        Box::new(Expression::Value(Value::Number(-2.0))),
        Box::new(Expression::Add(
            Box::new(Expression::Value(Value::Fraction(
                Fraction::new(2.0, 3.0).unwrap(),
            ))),
            Box::new(Expression::Value(Value::Number(4.0))),
        )),
    );

    println!("{:#?}", expression.evaluate());
}
