use core::fmt::Debug;
use std::str::FromStr;
use std::{error, fmt};

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Operand,
    Operator,
    OpenBrace,
    CloseBrace,
    ArgSeparator,
}

#[derive(PartialEq, Copy, Clone)]
pub enum OperatorResult {
    F64(f64),
    Bool(bool),
}

#[derive(PartialEq, Debug, Clone)]
pub enum EvaluationParseError {
    UnsupportedValue(String),
    InvalidExpression(String),
}

impl error::Error for EvaluationParseError {}

impl fmt::Display for EvaluationParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvaluationParseError::UnsupportedValue(e) => {
                write!(
                    f,
                    "value '{}' not supported, only f64 and bool are supported",
                    e
                )
            }
            EvaluationParseError::InvalidExpression(e) => {
                write!(f, "the expression '{}' is invalid", e)
            }
        }
    }
}

impl Debug for OperatorResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperatorResult::F64(v) => write!(f, "{}", v),
            OperatorResult::Bool(v) => write!(f, "{}", v),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub t: TokenType,
    pub v: String,
}

pub trait Operator<T> {
    fn eval(&self) -> T;
    fn to_string(&self) -> String;
}

impl<T> Debug for dyn Operator<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub struct Value {
    val: OperatorResult,
}

impl Operator<OperatorResult> for Value {
    fn eval(&self) -> OperatorResult {
        self.val
    }

    fn to_string(&self) -> String {
        format!("{:?}", self.val)
    }
}

impl FromStr for Value {
    type Err = EvaluationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = f64::from_str(s) {
            return Ok(Self {
                val: OperatorResult::F64(val),
            });
        }
        if let Ok(val) = bool::from_str(s) {
            return Ok(Self {
                val: OperatorResult::Bool(val),
            });
        }
        Err(EvaluationParseError::UnsupportedValue(s.into()))
    }
}

#[derive(Debug)]
pub struct GenericOperator<F> {
    left: Box<dyn Operator<OperatorResult>>,
    right: Box<dyn Operator<OperatorResult>>,
    func: F,
    op: String,
}

impl<F> GenericOperator<F> {
    pub fn new(
        left: Box<dyn Operator<OperatorResult>>,
        right: Box<dyn Operator<OperatorResult>>,
        func: F,
        op: String,
    ) -> Self {
        Self {
            left,
            right,
            func,
            op,
        }
    }
}

impl Operator<OperatorResult> for GenericOperator<fn(f64, f64) -> OperatorResult> {
    fn eval(&self) -> OperatorResult {
        let left = match self.left.eval() {
            OperatorResult::F64(v) => v,
            _ => f64::NAN,
        };
        let right = match self.right.eval() {
            OperatorResult::F64(v) => v,
            _ => f64::NAN,
        };
        (self.func)(left, right)
    }

    fn to_string(&self) -> String {
        format!("({:?} {} {:?})", self.left, self.op, self.right)
    }
}

impl Operator<OperatorResult> for GenericOperator<fn(bool, bool) -> OperatorResult> {
    fn eval(&self) -> OperatorResult {
        let left = match self.left.eval() {
            OperatorResult::Bool(v) => v,
            _ => false,
        };
        let right = match self.right.eval() {
            OperatorResult::Bool(v) => v,
            _ => false,
        };
        (self.func)(left, right)
    }

    fn to_string(&self) -> String {
        format!("({:?} {} {:?})", self.left, self.op, self.right)
    }
}
