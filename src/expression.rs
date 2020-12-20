use std::str::FromStr;

use crate::evaluable::{BasicEvaluable, Evaluable, EvaluableResult, EvaluationParseError, Value};
use crate::parser;
use crate::parser::TokenType::*;

#[derive(Debug)]
pub struct Expression<T> {
    evaluable: Box<dyn Evaluable<T>>,
}

impl Evaluable<EvaluableResult> for Expression<EvaluableResult> {
    fn eval(&self) -> EvaluableResult {
        self.evaluable.eval()
    }

    fn to_string(&self) -> String {
        format!("{:?}", self.evaluable)
    }
}

impl FromStr for Expression<EvaluableResult> {
    type Err = EvaluationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rpn = parser::to_rpn(parser::str_to_tokens(s));
        let mut stack: Vec<Box<dyn Evaluable<_>>> = Vec::new();
        for token in rpn {
            match token.t {
                Operand => stack.push(Box::new(Value::from_str(&token.v)?)),
                Operator => {
                    if stack.len() < 2 {
                        return Err(EvaluationParseError::InvalidExpression(s.into()));
                    }
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    let mut f64_func: Option<fn(f64, f64) -> EvaluableResult> = None;
                    let mut bool_func: Option<fn(bool, bool) -> EvaluableResult> = None;
                    if token.v == "+" {
                        f64_func = Some(|a, b| EvaluableResult::F64(a + b));
                    } else if token.v == "-" {
                        f64_func = Some(|a, b| EvaluableResult::F64(a - b));
                    } else if token.v == "*" {
                        f64_func = Some(|a, b| EvaluableResult::F64(a * b));
                    } else if token.v == "/" {
                        f64_func = Some(|a, b| EvaluableResult::F64(a / b));
                    } else if token.v == "%" {
                        f64_func = Some(|a, b| EvaluableResult::F64(a % b));
                    } else if token.v == "^" {
                        f64_func = Some(|a, b| EvaluableResult::F64(a.powf(b)));
                    } else if token.v == "<" {
                        f64_func = Some(|a, b| EvaluableResult::Bool(a < b));
                    } else if token.v == "<=" {
                        f64_func = Some(|a, b| EvaluableResult::Bool(a <= b));
                    } else if token.v == ">" {
                        f64_func = Some(|a, b| EvaluableResult::Bool(a > b));
                    } else if token.v == ">=" {
                        f64_func = Some(|a, b| EvaluableResult::Bool(a >= b));
                    } else if token.v == "&&" {
                        bool_func = Some(|a, b| EvaluableResult::Bool(a && b));
                    } else if token.v == "||" {
                        bool_func = Some(|a, b| EvaluableResult::Bool(a || b));
                    }
                    if let Some(func) = f64_func {
                        stack.push(Box::new(BasicEvaluable::new(left, right, func, token.v)));
                    } else if let Some(func) = bool_func {
                        stack.push(Box::new(BasicEvaluable::new(left, right, func, token.v)));
                    }
                }
                _ => return Err(EvaluationParseError::InvalidExpression(s.into())),
            }
        }
        Ok(Expression {
            evaluable: stack.pop().unwrap(),
        })
    }
}

#[cfg(test)]
mod expression_tests {
    use super::*;

    #[test]
    fn numeric_eval_1() {
        let e = <Expression<EvaluableResult>>::from_str("(6+10.0-4)/(1+1*2)+1").unwrap();
        assert_eq!(EvaluableResult::F64(5.0), e.eval());
    }

    #[test]
    fn numeric_eval_2() {
        let e = <Expression<EvaluableResult>>::from_str("(6>7)+(5>6)").unwrap();
        match e.eval() {
            EvaluableResult::F64(v) => assert!(v.is_nan()),
            EvaluableResult::Bool(_) => assert!(false),
        }
    }

    #[test]
    fn bool_eval_1() {
        let e = <Expression<EvaluableResult>>::from_str("(6+10-4)/(1+1*2)+1>6").unwrap();
        assert_eq!(EvaluableResult::Bool(false), e.eval());
    }

    #[test]
    fn bool_eval_2() {
        let e = <Expression<EvaluableResult>>::from_str("(6+10-4)/(1+1*2)+1>4 && 7>6").unwrap();
        assert_eq!(EvaluableResult::Bool(true), e.eval());
    }

    #[test]
    fn bool_eval_4() {
        let e = <Expression<EvaluableResult>>::from_str("(6+10-4)/(1+1*2)+1>4 && 7>6 && false")
            .unwrap();
        assert_eq!(EvaluableResult::Bool(false), e.eval());
    }

    #[test]
    fn bool_eval_5() {
        let e = <Expression<EvaluableResult>>::from_str("6 && 6").unwrap();
        assert_eq!(EvaluableResult::Bool(false), e.eval());
    }

    #[test]
    fn f64_to_string() {
        let e = <Expression<EvaluableResult>>::from_str("6+6").unwrap();
        assert_eq!("(6 + 6)", e.to_string());
    }

    #[test]
    fn bool_to_string() {
        let e = <Expression<EvaluableResult>>::from_str("5>6&&4>6").unwrap();
        assert_eq!("((5 > 6) && (4 > 6))", e.to_string());
    }

    #[test]
    fn incorrect_eval_1() {
        let e = <Expression<EvaluableResult>>::from_str("6+");
        assert_eq!(
            EvaluationParseError::InvalidExpression("6+".into()),
            e.expect_err("no error returned")
        );
    }

    #[test]
    fn incorrect_eval_2() {
        let e = <Expression<EvaluableResult>>::from_str("(6+6");
        assert_eq!(
            EvaluationParseError::InvalidExpression("(6+6".into()),
            e.expect_err("no error returned")
        );
    }

    #[test]
    fn incorrect_eval_3() {
        let e = <Expression<EvaluableResult>>::from_str("+6");
        assert_eq!(
            EvaluationParseError::InvalidExpression("+6".into()),
            e.expect_err("no error returned")
        );
    }

    #[test]
    fn unsupported_value() {
        let e = <Expression<EvaluableResult>>::from_str("6as");
        assert_eq!(
            EvaluationParseError::UnsupportedValue("6as".into()),
            e.expect_err("no error returned")
        );
    }

    #[test]
    fn plus() {
        let e = <Expression<EvaluableResult>>::from_str("5+5").unwrap();
        assert_eq!(EvaluableResult::F64(10.0), e.eval());
    }

    #[test]
    fn minus() {
        let e = <Expression<EvaluableResult>>::from_str("5-5").unwrap();
        assert_eq!(EvaluableResult::F64(0.0), e.eval());
    }

    #[test]
    fn multiply() {
        let e = <Expression<EvaluableResult>>::from_str("5*5").unwrap();
        assert_eq!(EvaluableResult::F64(25.0), e.eval());
    }

    #[test]
    fn divide() {
        let e = <Expression<EvaluableResult>>::from_str("5/5").unwrap();
        assert_eq!(EvaluableResult::F64(1.0), e.eval());
    }

    #[test]
    fn pow() {
        let e = <Expression<EvaluableResult>>::from_str("5^5").unwrap();
        assert_eq!(EvaluableResult::F64(3125.0), e.eval());
    }

    #[test]
    fn mod_op() {
        let e = <Expression<EvaluableResult>>::from_str("27%5").unwrap();
        assert_eq!(EvaluableResult::F64(2.0), e.eval());
    }

    #[test]
    fn less() {
        let e = <Expression<EvaluableResult>>::from_str("5<6").unwrap();
        assert_eq!(EvaluableResult::Bool(true), e.eval());
    }

    #[test]
    fn greater() {
        let e = <Expression<EvaluableResult>>::from_str("6>5").unwrap();
        assert_eq!(EvaluableResult::Bool(true), e.eval());
    }

    #[test]
    fn greater_or_equal() {
        let e = <Expression<EvaluableResult>>::from_str("5>=5").unwrap();
        assert_eq!(EvaluableResult::Bool(true), e.eval());
    }

    #[test]
    fn less_or_equal() {
        let e = <Expression<EvaluableResult>>::from_str("5<=5").unwrap();
        assert_eq!(EvaluableResult::Bool(true), e.eval());
    }

    #[test]
    fn logical_and() {
        let e = <Expression<EvaluableResult>>::from_str("5<=5 && 6>=5").unwrap();
        assert_eq!(EvaluableResult::Bool(true), e.eval());
    }

    #[test]
    fn logical_or() {
        let e = <Expression<EvaluableResult>>::from_str("5<2 || 6>=5").unwrap();
        assert_eq!(EvaluableResult::Bool(true), e.eval());
    }
}
