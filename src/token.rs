use std::str::FromStr;
use core::fmt::Debug;
use std::fmt;

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
}

#[derive(PartialEq)]
pub struct Value {
    val: OperatorResult,
}

impl Operator<OperatorResult> for Value {
    fn eval(&self) -> OperatorResult {
        self.val
    }
}

impl FromStr for Value {

    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = f64::from_str(s) {
            return Ok(Self{val:OperatorResult::F64(val)})
        }
        if let Ok(val) = bool::from_str(s) {
            return Ok(Self{val:OperatorResult::Bool(val)})
        }
        Err("Unsupported type".into())
    }
}

pub struct F64Operator<F> {
    left: Box<dyn Operator<OperatorResult>>,
    right: Box<dyn Operator<OperatorResult>>,
    func: F,
}

impl<F> F64Operator<F> {
    pub fn new(left: Box<dyn Operator<OperatorResult>>, right: Box<dyn Operator<OperatorResult>>, func: F) -> Self {
        Self { left, right, func }
    }
}

impl<F> Operator<OperatorResult> for F64Operator<F>
    where F: Fn(f64, f64) -> OperatorResult {
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
}

pub struct BoolOperator<F> {
    left: Box<dyn Operator<OperatorResult>>,
    right: Box<dyn Operator<OperatorResult>>,
    func: F,
}

impl<F> BoolOperator<F> {
    pub fn new(left: Box<dyn Operator<OperatorResult>>, right: Box<dyn Operator<OperatorResult>>, func: F) -> Self {
        Self { left, right, func }
    }
}

impl<F> Operator<OperatorResult> for BoolOperator<F>
    where F: Fn(bool, bool) -> OperatorResult {
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
}