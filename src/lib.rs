use std::convert::TryFrom;

use crate::token::{AddOperator, Evaluable, Value};

mod token;

pub fn str_to_tokens(s: &str) -> Vec<&str> {
    let mut start = 0;
    let mut end = 0;
    let mut chars = s.chars();
    let mut tokens: Vec<&str> = Vec::new();
    loop {
        let mut c = match chars.next() {
            None => ' ',
            Some(c) => c,
        };
        if c == ' ' { break; }
        if c.is_numeric() {
            while c.is_numeric() || c == '.' {
                c = match chars.next() {
                    None => ' ',
                    Some(c) => c,
                };
                end += 1;
            }
            let sub = &s[start..end];
            tokens.push(sub);
            start = end;
        }
        if c == '+' {
            tokens.push("+");
            start += 1;
            end += 1;
        }
    }
    tokens
}

fn to_rpn(tokens: Vec<&str>) -> Vec<&str> {
    let mut rpn = Vec::new();
    let mut ops = Vec::new();
    for token in tokens {
        if token == "(" {
            ops.push("(");
        } else if token == ")" {
            while let Some(op) = ops.pop() {
                if op == "(" { break; }
                rpn.push(op);
            }
        } else if !is_operator(token) {
            rpn.push(token);
        } else {
            if ops.is_empty() || get_operator_precedence(ops.last().unwrap()) > get_operator_precedence(token) {
                ops.push(token);
            } else {
                while let Some(stack_op) = ops.last() {
                    if get_operator_precedence(token) >= get_operator_precedence(stack_op) {
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

fn is_operator(op: &str) -> bool {
    op == "+" || op == "-" || op == "/" || op == "*"
}

fn get_operator_precedence(op: &str) -> u8 {
    return if op == "*" || op == "/" || op == "%" {
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
    } else if op == "^" {
        90
    } else if op == "|" {
        100
    } else if op == "&&" {
        110
    } else if op == "||" {
        120
    } else {
        160
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
    fn str_to_tokens_test() {
        str_to_tokens("123.23+100");
    }

    #[test]
    fn to_rpn_test_1() {
        assert_eq!(vec!["1", "2", "+"], to_rpn(vec!["1", "+", "2"]));
    }

    #[test]
    fn to_rpn_test_2() {
        assert_eq!(vec!["a", "b", "c", "*", "+", "d", "+"], to_rpn(vec!["a", "+", "b", "*", "c", "+", "d"]));
    }

    #[test]
    fn str_to_rpn_test_1() {
        assert_eq!(vec!["a", "b", "c", "*", "+", "d", "+"], to_rpn(str_to_tokens("a+b*c+d")));
    }
}