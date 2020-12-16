use std::convert::TryFrom;
use std::str::FromStr;

pub use crate::token::{AddOperator, DivideOperator, GreaterOperator, LogicalAndOperator, MultiplyOperator, Operator, SubtractOperator, Token, TokenType, Value};
pub use crate::token::OperatorResult;
use crate::token::TokenType::*;

mod token;

#[derive(Debug)]
pub struct Evaluable<T> {
    operator: Box<dyn Operator<T>>,
}

impl Operator<OperatorResult> for Evaluable<OperatorResult> {
    fn eval(&self) -> OperatorResult {
        self.operator.eval()
    }

    fn print(&self) -> String {
        format!("{:?}", self.operator)
    }
}

impl Operator<bool> for Evaluable<bool> {
    fn eval(&self) -> bool {
        self.operator.eval()
    }

    fn print(&self) -> String {
        format!("{:?}", self.operator)
    }
}

impl FromStr for Evaluable<OperatorResult> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rpn = to_rpn(str_to_tokens(s));
        let mut stack: Vec<Box<dyn Operator<_>>> = Vec::new();
        for token in rpn {
            match token.t {
                Operand => stack.push(Box::new(Value::from_str(&token.v).unwrap())),
                Operator => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    if token.v == "+" {
                        stack.push(Box::new(AddOperator::new(left, right)));
                    } else if token.v == "-" {
                        stack.push(Box::new(SubtractOperator::new(left, right)));
                    } else if token.v == "*" {
                        stack.push(Box::new(MultiplyOperator::new(left, right)));
                    } else if token.v == "/" {
                        stack.push(Box::new(DivideOperator::new(left, right)));
                    } else if token.v == ">" {
                        stack.push(Box::new(GreaterOperator::new(left, right)));
                    } else if token.v == "&&" {
                        stack.push(Box::new(LogicalAndOperator::new(left, right)));
                    }
                }
                _ => {}
            }
        }
        Ok(Evaluable { operator: stack.pop().unwrap() })
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
            tokens.push(Token { t: OpenBrace, v: String::from(c) })
        } else if c == ')' {
            tokens.push(Token { t: CloseBrace, v: String::from(c) })
        } else if c == ',' {
            tokens.push(Token { t: ArgSeparator, v: String::from(c) })
        } else if c == '*' || c == '/' || c == '%' || c == '+' || c == '-' || c == '<' || c == '>'
            || c == '=' || c == '!' || c == '&' || c == '^' || c == '|' {
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
        None => tokens.push(Token { t: token_type, v: String::from(p) + &String::from(c) }),
        Some(t) => {
            if t.t == token_type {
                t.v.push(c);
            } else {
                tokens.push(Token { t: token_type, v: String::from(p) + &String::from(c) });
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
                if op.t == OpenBrace { break; }
                rpn.push(op);
            }
        } else if token.t != Operator {
            rpn.push(token);
        } else {
            if ops.is_empty() || get_operator_precedence(&ops.last().unwrap().v) > get_operator_precedence(&token.v) {
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
    } else if op == "<<" || op == ">>" {
        50
    } else if op == "<" || op == "<=" || op == ">" || op == ">=" {
        60
    } else if op == "==" || op == "!=" {
        70
    } else if op == "&" {
        80
    } else if op == "|" {
        100
    } else if op == "&&" {
        110
    } else if op == "||" {
        120
    } else {
        u8::max_value()
    };
}

pub fn str_to_int(s: &str) -> i64 {
    let mut res: i64 = 0;
    let mut pow = u32::try_from(s.len()).expect("too long string") - 1;
    for c in s.chars() {
        let d = c.to_digit(10).expect("not a digit");
        res += i64::from(d) * 10_i64.pow(pow);
        if pow > 0 {
            pow -= 1;
        }
    }
    return res;
}

pub fn str_to_float(s: &str) -> f64 {
    let mut res = 0.0;
    let mut f_part = false;
    for part in s.split('.') {
        if f_part {
            res += str_to_int(part) as f64 * 1.0 / 10_i64.pow(part.len() as u32) as f64
        } else {
            res += str_to_int(part) as f64;
        }
        f_part = true;
    }
    return res;
}

#[cfg(test)]
mod str_to_tests {
    use super::*;

    #[test]
    fn str_to_int_test() {
        assert_eq!(123, str_to_int("123"));
        assert_eq!(0, str_to_int("0"));
    }

    #[test]
    fn str_to_float_test() {
        assert_eq!(123.0, str_to_float("123"));
        assert_eq!(0.0, str_to_float("0"));
        assert_eq!(123.123, str_to_float("123.123"));
        assert_eq!(0.123, str_to_float("0.123"));
    }

    #[test]
    fn str_to_rpn_test_1() {
        let rpn: String = to_rpn(str_to_tokens("a+b*c+d")).iter().flat_map(|t| t.v.chars()).collect();
        assert_eq!("abc*+d+", rpn);
    }

    #[test]
    fn str_to_rpn_test_2() {
        let rpn: String = to_rpn(str_to_tokens("a+b*(c^d-e)^(f+g*h)-i")).iter().flat_map(|t| t.v.chars()).collect();
        assert_eq!("abcd^e-fgh*+^*+i-", rpn);
    }

    #[test]
    fn str_to_rpn_test_3() {
        let rpn: String = to_rpn(str_to_tokens("A*(B+C)/D")).iter().flat_map(|t| t.v.chars()).collect();
        assert_eq!("ABC+*D/", rpn);
    }

    #[test]
    fn str_to_rpn_test_4() {
        let rpn: String = to_rpn(str_to_tokens("(6+10-4)/(1+1*2)+1")).iter().flat_map(|t| t.v.chars()).collect();
        assert_eq!("610+4-112*+/1+", rpn);
    }

    #[test]
    fn numeric_eval() {
        let e = <Evaluable<OperatorResult>>::from_str("(6+10-4)/(1+1*2)+1").unwrap();
        println!("evaluable: {:?}", e);
        assert_eq!(OperatorResult::F64(5.0), e.eval());
    }

    #[test]
    fn bool_eval() {
        let e = <Evaluable<OperatorResult>>::from_str("(6+10-4)/(1+1*2)+1>6").unwrap();
        println!("evaluable: {:?}", e);
        assert_eq!(OperatorResult::Bool(false), e.eval());
    }
}