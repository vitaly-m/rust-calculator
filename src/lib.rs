mod token;

use std::convert::TryFrom;
use crate::token::{Value, Token, AddOperator};
use crate::token::Token::{Operand, Add};

pub fn str_to_tokens(s: &str) {
    let mut start = 0;
    let mut end = 0;
    let mut chars = s.chars();
    let mut tokens: Vec<Token> = Vec::new();
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
            let token = <Value<f64>>::new(sub).unwrap();
            tokens.push(Operand(token));
            start = end;
        }
        if c == '+' {
            tokens.push(Add(AddOperator{}));
        }
    }
    println!("tokens: {:?}", tokens);
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
}