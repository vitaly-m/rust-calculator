use std::str::FromStr;
use core::fmt::Debug;
use std::fmt::Formatter;
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
    fn print(&self) -> String;
}

impl<T> Debug for dyn Operator<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{{}}}", self.print())
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

    fn print(&self) -> String {
        format!("{:?}", self.val)
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

#[derive(Debug)]
pub struct AddOperator {
    left: Box<dyn Operator<OperatorResult>>,
    right: Box<dyn Operator<OperatorResult>>,
}

impl AddOperator {
    pub fn new(left: Box<dyn Operator<OperatorResult>>, right: Box<dyn Operator<OperatorResult>>) -> Self {
        Self { left, right }
    }
}

impl Operator<OperatorResult> for AddOperator {
    fn eval(&self) -> OperatorResult {
        let left = match self.left.eval() {
            OperatorResult::F64(v) => v,
            _ => f64::NAN,
        };
        let right = match self.right.eval() {
            OperatorResult::F64(v) => v,
            _ => f64::NAN,
        };
        OperatorResult::F64(left + right)
    }

    fn print(&self) -> String {
        format!("({} + {})", self.left.print(), self.right.print())
    }
}

#[derive(Debug)]
pub struct SubtractOperator {
    left: Box<dyn Operator<OperatorResult>>,
    right: Box<dyn Operator<OperatorResult>>,
}

impl SubtractOperator {
    pub fn new(left: Box<dyn Operator<OperatorResult>>, right: Box<dyn Operator<OperatorResult>>) -> Self {
        Self { left, right }
    }
}

impl Operator<OperatorResult> for SubtractOperator {
    fn eval(&self) -> OperatorResult {
        let left = match self.left.eval() {
            OperatorResult::F64(v) => v,
            _ => f64::NAN,
        };
        let right = match self.right.eval() {
            OperatorResult::F64(v) => v,
            _ => f64::NAN,
        };
        OperatorResult::F64(left - right)
    }

    fn print(&self) -> String {
        format!("({} - {})", self.left.print(), self.right.print())
    }
}

#[derive(Debug)]
pub struct MultiplyOperator {
    left: Box<dyn Operator<OperatorResult>>,
    right: Box<dyn Operator<OperatorResult>>,
}

impl MultiplyOperator {
    pub fn new(left: Box<dyn Operator<OperatorResult>>, right: Box<dyn Operator<OperatorResult>>) -> Self {
        Self { left, right }
    }
}

impl Operator<OperatorResult> for MultiplyOperator {
    fn eval(&self) -> OperatorResult {
        let left = match self.left.eval() {
            OperatorResult::F64(v) => v,
            _ => f64::NAN,
        };
        let right = match self.right.eval() {
            OperatorResult::F64(v) => v,
            _ => f64::NAN,
        };
        OperatorResult::F64(left * right)
    }

    fn print(&self) -> String {
        format!("({} * {})", self.left.print(), self.right.print())
    }
}

#[derive(Debug)]
pub struct DivideOperator {
    left: Box<dyn Operator<OperatorResult>>,
    right: Box<dyn Operator<OperatorResult>>,
}

impl DivideOperator {
    pub fn new(left: Box<dyn Operator<OperatorResult>>, right: Box<dyn Operator<OperatorResult>>) -> Self {
        Self { left, right }
    }
}

impl Operator<OperatorResult> for DivideOperator {
    fn eval(&self) -> OperatorResult {
        let left = match self.left.eval() {
            OperatorResult::F64(v) => v,
            _ => f64::NAN,
        };
        let right = match self.right.eval() {
            OperatorResult::F64(v) => v,
            _ => f64::NAN,
        };
        OperatorResult::F64(left / right)
    }

    fn print(&self) -> String {
        format!("({} / {})", self.left.print(), self.right.print())
    }
}

#[derive(Debug)]
pub struct GreaterOperator {
    left: Box<dyn Operator<OperatorResult>>,
    right: Box<dyn Operator<OperatorResult>>,
}

impl GreaterOperator {
    pub fn new(left: Box<dyn Operator<OperatorResult>>, right: Box<dyn Operator<OperatorResult>>) -> Self {
        Self { left, right }
    }
}

impl Operator<OperatorResult> for GreaterOperator {
    fn eval(&self) -> OperatorResult {
        let left = match self.left.eval() {
            OperatorResult::F64(v) => v,
            _ => f64::NAN,
        };
        let right = match self.right.eval() {
            OperatorResult::F64(v) => v,
            _ => f64::NAN,
        };
        OperatorResult::Bool(left > right)
    }

    fn print(&self) -> String {
        format!("({} > {})", self.left.print(), self.right.print())
    }
}

#[derive(Debug)]
pub struct LogicalAndOperator {
    left: Box<dyn Operator<OperatorResult>>,
    right: Box<dyn Operator<OperatorResult>>,
}

impl LogicalAndOperator {
    pub fn new(left: Box<dyn Operator<OperatorResult>>, right: Box<dyn Operator<OperatorResult>>) -> Self {
        Self { left, right }
    }
}

impl Operator<OperatorResult> for LogicalAndOperator {
    fn eval(&self) -> OperatorResult {
        let left = match self.left.eval() {
            OperatorResult::Bool(v) => v,
            _ => false,
        };
        let right = match self.right.eval() {
            OperatorResult::Bool(v) => v,
            _ => false,
        };
        OperatorResult::Bool(left && right)
    }

    fn print(&self) -> String {
        format!("({} && {})", self.left.print(), self.right.print())
    }
}