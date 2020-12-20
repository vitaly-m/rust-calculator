use std::str::FromStr;

pub use crate::token::OperatorResult;
use crate::token::TokenType::*;
use crate::token::{EvaluationParseError, GenericOperator};
pub use crate::token::{Operator, Token, TokenType, Value};

mod token;

#[derive(Debug)]
pub struct Evaluable<T> {
    operator: Box<dyn Operator<T>>,
}

impl Operator<OperatorResult> for Evaluable<OperatorResult> {
    fn eval(&self) -> OperatorResult {
        self.operator.eval()
    }

    fn to_string(&self) -> String {
        format!("{:?}", self.operator)
    }
}

impl FromStr for Evaluable<OperatorResult> {
    type Err = EvaluationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rpn = to_rpn(str_to_tokens(s));
        let mut stack: Vec<Box<dyn Operator<_>>> = Vec::new();
        for token in rpn {
            match token.t {
                Operand => stack.push(Box::new(Value::from_str(&token.v)?)),
                Operator => {
                    if stack.len() < 2 {
                        return Err(EvaluationParseError::InvalidExpression(s.into()));
                    }
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    let mut f64_func: Option<fn(f64, f64) -> OperatorResult> = None;
                    let mut bool_func: Option<fn(bool, bool) -> OperatorResult> = None;
                    if token.v == "+" {
                        f64_func = Some(|a, b| OperatorResult::F64(a + b));
                    } else if token.v == "-" {
                        f64_func = Some(|a, b| OperatorResult::F64(a - b));
                    } else if token.v == "*" {
                        f64_func = Some(|a, b| OperatorResult::F64(a * b));
                    } else if token.v == "/" {
                        f64_func = Some(|a, b| OperatorResult::F64(a / b));
                    } else if token.v == "%" {
                        f64_func = Some(|a, b| OperatorResult::F64(a % b));
                    } else if token.v == "^" {
                        f64_func = Some(|a, b| OperatorResult::F64(a.powf(b)));
                    } else if token.v == "<" {
                        f64_func = Some(|a, b| OperatorResult::Bool(a < b));
                    } else if token.v == "<=" {
                        f64_func = Some(|a, b| OperatorResult::Bool(a <= b));
                    } else if token.v == ">" {
                        f64_func = Some(|a, b| OperatorResult::Bool(a > b));
                    } else if token.v == ">=" {
                        f64_func = Some(|a, b| OperatorResult::Bool(a >= b));
                    } else if token.v == "&&" {
                        bool_func = Some(|a, b| OperatorResult::Bool(a && b));
                    } else if token.v == "||" {
                        bool_func = Some(|a, b| OperatorResult::Bool(a || b));
                    }
                    if let Some(func) = f64_func {
                        stack.push(Box::new(GenericOperator::new(left, right, func, token.v)));
                    } else if let Some(func) = bool_func {
                        stack.push(Box::new(GenericOperator::new(left, right, func, token.v)));
                    }
                }
                _ => return Err(EvaluationParseError::InvalidExpression(s.into())),
            }
        }
        Ok(Evaluable {
            operator: stack.pop().unwrap(),
        })
    }
}

pub fn str_to_tokens(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for c in s.chars() {
        if c.is_alphanumeric() {
            process_char(&mut tokens, c, Operand, None);
        } else if c == '.' {
            process_char(&mut tokens, c, Operand, Some("0"));
        } else if c == '(' {
            tokens.push(Token {
                t: OpenBrace,
                v: String::from(c),
            })
        } else if c == ')' {
            tokens.push(Token {
                t: CloseBrace,
                v: String::from(c),
            })
        } else if c == ',' {
            tokens.push(Token {
                t: ArgSeparator,
                v: String::from(c),
            })
        } else if c == '*'
            || c == '/'
            || c == '%'
            || c == '+'
            || c == '-'
            || c == '<'
            || c == '>'
            || c == '='
            || c == '!'
            || c == '&'
            || c == '^'
            || c == '|'
        {
            process_char(&mut tokens, c, Operator, None);
        }
    }
    tokens
}

fn process_char(tokens: &mut Vec<Token>, c: char, token_type: TokenType, prefix: Option<&str>) {
    let p = match prefix {
        None => "",
        Some(pp) => pp,
    };
    match tokens.last_mut() {
        None => tokens.push(Token {
            t: token_type,
            v: String::from(p) + &String::from(c),
        }),
        Some(t) => {
            if t.t == token_type {
                t.v.push(c);
            } else {
                tokens.push(Token {
                    t: token_type,
                    v: String::from(p) + &String::from(c),
                });
            }
        }
    };
}

fn to_rpn(tokens: Vec<Token>) -> Vec<Token> {
    let mut rpn = Vec::new();
    let mut ops = Vec::new();
    for token in tokens {
        if token.t == OpenBrace {
            ops.push(token);
        } else if token.t == CloseBrace {
            while let Some(op) = ops.pop() {
                if op.t == OpenBrace {
                    break;
                }
                rpn.push(op);
            }
        } else if token.t != Operator {
            rpn.push(token);
        } else {
            if ops.is_empty()
                || get_operator_precedence(&ops.last().unwrap().v)
                    > get_operator_precedence(&token.v)
            {
                ops.push(token);
            } else {
                while let Some(stack_op) = ops.last() {
                    if get_operator_precedence(&token.v) >= get_operator_precedence(&stack_op.v) {
                        rpn.push(ops.pop().unwrap());
                    } else {
                        break;
                    }
                }
                ops.push(token);
            }
        }
    }
    while let Some(op) = ops.pop() {
        rpn.push(op);
    }
    rpn
}

fn get_operator_precedence(op: &str) -> u8 {
    return if op == "^" {
        29
    } else if op == "*" || op == "/" || op == "%" {
        30
    } else if op == "+" || op == "-" {
        40
    } else if op == "<" || op == "<=" || op == ">" || op == ">=" {
        60
    } else if op == "&&" {
        110
    } else if op == "||" {
        120
    } else {
        u8::max_value()
    };
}

#[cfg(test)]
mod str_to_tests {
    use super::*;

    #[test]
    fn str_to_rpn_test_1() {
        let rpn: String = to_rpn(str_to_tokens("a+b*c+d"))
            .iter()
            .flat_map(|t| t.v.chars())
            .collect();
        assert_eq!("abc*+d+", rpn);
    }

    #[test]
    fn str_to_rpn_test_2() {
        let rpn: String = to_rpn(str_to_tokens("a+b*(c^d-e)^(f+g*h)-i"))
            .iter()
            .flat_map(|t| t.v.chars())
            .collect();
        assert_eq!("abcd^e-fgh*+^*+i-", rpn);
    }

    #[test]
    fn str_to_rpn_test_3() {
        let rpn: String = to_rpn(str_to_tokens("A*(B+C)/D"))
            .iter()
            .flat_map(|t| t.v.chars())
            .collect();
        assert_eq!("ABC+*D/", rpn);
    }

    #[test]
    fn str_to_rpn_test_4() {
        let rpn: String = to_rpn(str_to_tokens("(6+10-4)/(1+1*2)+1"))
            .iter()
            .flat_map(|t| t.v.chars())
            .collect();
        assert_eq!("610+4-112*+/1+", rpn);
    }

    #[test]
    fn numeric_eval_1() {
        let e = <Evaluable<OperatorResult>>::from_str("(6+10.0-4)/(1+1*2)+1").unwrap();
        assert_eq!(OperatorResult::F64(5.0), e.eval());
    }

    #[test]
    fn numeric_eval_2() {
        let e = <Evaluable<OperatorResult>>::from_str("(6>7)+(5>6)").unwrap();
        match e.eval() {
            OperatorResult::F64(v) => assert!(v.is_nan()),
            OperatorResult::Bool(_) => assert!(false),
        }
    }

    #[test]
    fn bool_eval_1() {
        let e = <Evaluable<OperatorResult>>::from_str("(6+10-4)/(1+1*2)+1>6").unwrap();
        assert_eq!(OperatorResult::Bool(false), e.eval());
    }

    #[test]
    fn bool_eval_2() {
        let e = <Evaluable<OperatorResult>>::from_str("(6+10-4)/(1+1*2)+1>4 && 7>6").unwrap();
        assert_eq!(OperatorResult::Bool(true), e.eval());
    }

    #[test]
    fn bool_eval_4() {
        let e =
            <Evaluable<OperatorResult>>::from_str("(6+10-4)/(1+1*2)+1>4 && 7>6 && false").unwrap();
        assert_eq!(OperatorResult::Bool(false), e.eval());
    }

    #[test]
    fn bool_eval_5() {
        let e = <Evaluable<OperatorResult>>::from_str("6 && 6").unwrap();
        assert_eq!(OperatorResult::Bool(false), e.eval());
    }

    #[test]
    fn f64_to_string() {
        let e = <Evaluable<OperatorResult>>::from_str("6+6").unwrap();
        assert_eq!("(6 + 6)", e.to_string());
    }

    #[test]
    fn bool_to_string() {
        let e = <Evaluable<OperatorResult>>::from_str("5>6&&4>6").unwrap();
        assert_eq!("((5 > 6) && (4 > 6))", e.to_string());
    }

    #[test]
    fn incorrect_eval_1() {
        let e = <Evaluable<OperatorResult>>::from_str("6+");
        assert_eq!(
            EvaluationParseError::InvalidExpression("6+".into()),
            e.expect_err("no error returned")
        );
    }

    #[test]
    fn incorrect_eval_2() {
        let e = <Evaluable<OperatorResult>>::from_str("(6+6");
        assert_eq!(
            EvaluationParseError::InvalidExpression("(6+6".into()),
            e.expect_err("no error returned")
        );
    }

    #[test]
    fn incorrect_eval_3() {
        let e = <Evaluable<OperatorResult>>::from_str("+6");
        assert_eq!(
            EvaluationParseError::InvalidExpression("+6".into()),
            e.expect_err("no error returned")
        );
    }

    #[test]
    fn unsupported_value() {
        let e = <Evaluable<OperatorResult>>::from_str("6as");
        assert_eq!(
            EvaluationParseError::UnsupportedValue("6as".into()),
            e.expect_err("no error returned")
        );
    }
}

#[cfg(test)]
mod supported_operators_tests {
    use super::*;

    #[test]
    fn plus() {
        let e = <Evaluable<OperatorResult>>::from_str("5+5").unwrap();
        assert_eq!(OperatorResult::F64(10.0), e.eval());
    }

    #[test]
    fn minus() {
        let e = <Evaluable<OperatorResult>>::from_str("5-5").unwrap();
        assert_eq!(OperatorResult::F64(0.0), e.eval());
    }

    #[test]
    fn multiply() {
        let e = <Evaluable<OperatorResult>>::from_str("5*5").unwrap();
        assert_eq!(OperatorResult::F64(25.0), e.eval());
    }

    #[test]
    fn divide() {
        let e = <Evaluable<OperatorResult>>::from_str("5/5").unwrap();
        assert_eq!(OperatorResult::F64(1.0), e.eval());
    }

    #[test]
    fn pow() {
        let e = <Evaluable<OperatorResult>>::from_str("5^5").unwrap();
        assert_eq!(OperatorResult::F64(3125.0), e.eval());
    }

    #[test]
    fn mod_op() {
        let e = <Evaluable<OperatorResult>>::from_str("27%5").unwrap();
        assert_eq!(OperatorResult::F64(2.0), e.eval());
    }

    #[test]
    fn less() {
        let e = <Evaluable<OperatorResult>>::from_str("5<6").unwrap();
        assert_eq!(OperatorResult::Bool(true), e.eval());
    }

    #[test]
    fn greater() {
        let e = <Evaluable<OperatorResult>>::from_str("6>5").unwrap();
        assert_eq!(OperatorResult::Bool(true), e.eval());
    }

    #[test]
    fn greater_or_equal() {
        let e = <Evaluable<OperatorResult>>::from_str("5>=5").unwrap();
        assert_eq!(OperatorResult::Bool(true), e.eval());
    }

    #[test]
    fn less_or_equal() {
        let e = <Evaluable<OperatorResult>>::from_str("5<=5").unwrap();
        assert_eq!(OperatorResult::Bool(true), e.eval());
    }

    #[test]
    fn logical_and() {
        let e = <Evaluable<OperatorResult>>::from_str("5<=5 && 6>=5").unwrap();
        assert_eq!(OperatorResult::Bool(true), e.eval());
    }

    #[test]
    fn logical_or() {
        let e = <Evaluable<OperatorResult>>::from_str("5<2 || 6>=5").unwrap();
        assert_eq!(OperatorResult::Bool(true), e.eval());
    }
}
