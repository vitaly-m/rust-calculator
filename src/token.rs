use std::str::FromStr;

#[derive(Debug)]
pub enum Token {
    Operand(Value<f64>),
    Add(AddOperator),
}

pub trait OperandToken<T> {
    fn value(&self) -> T;
}

pub trait OperatorToken<T, R>
    where T: OperandToken<R> {
    fn eval(l: &T, r: &T) -> R;
}

#[derive(Debug, PartialEq)]
pub struct Value<T>
    where T: FromStr {
    val: T,
}

// impl<T> Token for Value<T>
//     where T: FromStr {
//     fn get_type() -> TokenType {
//         Operand
//     }
// }

impl<T> OperandToken<T> for Value<T>
    where T: FromStr + Copy{
    fn value(&self) -> T {
        self.val
    }
}

impl<T> Value<T>
    where T: FromStr {
    pub fn new(s: &str) -> Result<Self, <T as FromStr>::Err> {
        let v = T::from_str(s)?;
        Ok(Self { val: v })
    }
}

#[derive(Debug)]
pub struct AddOperator {}

// impl Token for AddOperator {
//     fn get_type() -> TokenType {
//         Operator
//     }
// }

impl OperatorToken<Value<f64>, f64> for AddOperator{

    fn eval(left: &Value<f64>, right: &Value<f64>) -> f64 {
        left.value() + right.value()
    }
}