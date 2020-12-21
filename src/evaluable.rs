use core::fmt::Debug;
use std::str::FromStr;
use std::{error, fmt};

#[derive(PartialEq, Copy, Clone)]
pub enum EvaluableResult {
    F64(f64),
    Bool(bool),
}

impl Debug for EvaluableResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvaluableResult::F64(v) => write!(f, "{}", v),
            EvaluableResult::Bool(v) => write!(f, "{}", v),
        }
    }
}

pub trait Evaluable<T> {
    fn eval(&self) -> T;
    fn to_string(&self) -> String;
}

impl<T> Debug for dyn Evaluable<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_string())
    }
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
                    "value '{}' is not supported, only f64 and bool are supported",
                    e
                )
            }
            EvaluationParseError::InvalidExpression(e) => {
                write!(f, "the expression '{}' is invalid", e)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Value {
    val: EvaluableResult,
}

impl Evaluable<EvaluableResult> for Value {
    fn eval(&self) -> EvaluableResult {
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
                val: EvaluableResult::F64(val),
            });
        }
        if let Ok(val) = bool::from_str(s) {
            return Ok(Self {
                val: EvaluableResult::Bool(val),
            });
        }
        Err(EvaluationParseError::UnsupportedValue(s.into()))
    }
}

#[derive(Debug)]
pub struct BasicEvaluable<F> {
    left: Box<dyn Evaluable<EvaluableResult>>,
    right: Box<dyn Evaluable<EvaluableResult>>,
    func: F,
    op: String,
}

impl<F> BasicEvaluable<F> {
    pub fn new(
        left: Box<dyn Evaluable<EvaluableResult>>,
        right: Box<dyn Evaluable<EvaluableResult>>,
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

impl Evaluable<EvaluableResult> for BasicEvaluable<fn(f64, f64) -> EvaluableResult> {
    fn eval(&self) -> EvaluableResult {
        let left = match self.left.eval() {
            EvaluableResult::F64(v) => v,
            _ => f64::NAN,
        };
        let right = match self.right.eval() {
            EvaluableResult::F64(v) => v,
            _ => f64::NAN,
        };
        (self.func)(left, right)
    }

    fn to_string(&self) -> String {
        format!("({:?} {} {:?})", self.left, self.op, self.right)
    }
}

impl Evaluable<EvaluableResult> for BasicEvaluable<fn(bool, bool) -> EvaluableResult> {
    fn eval(&self) -> EvaluableResult {
        let left = match self.left.eval() {
            EvaluableResult::Bool(v) => v,
            _ => false,
        };
        let right = match self.right.eval() {
            EvaluableResult::Bool(v) => v,
            _ => false,
        };
        (self.func)(left, right)
    }

    fn to_string(&self) -> String {
        format!("({:?} {} {:?})", self.left, self.op, self.right)
    }
}
