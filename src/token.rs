use std::str::FromStr;
use core::fmt::Debug;

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Operand,
    Operator,
    OpenBrace,
    CloseBrace,
    ArgSeparator,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub t: TokenType,
    pub v: String,
}

pub trait Evaluable<T> {
    fn eval(&self) -> T;
    fn print(&self) -> String;
}

impl<T> Debug for dyn Evaluable<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Token{{{}}}", self.print())
    }
}

#[derive(Debug, PartialEq)]
pub struct Value<T>
    where T: FromStr {
    val: T,
}

impl<T> Evaluable<T> for Value<T>
    where T: FromStr + Copy + ToString {
    fn eval(&self) -> T {
        self.val
    }

    fn print(&self) -> String {
        self.val.to_string()
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
pub struct AddOperator {
    left: Box<dyn Evaluable<f64>>,
    right: Box<dyn Evaluable<f64>>,
}

impl AddOperator {
    pub fn new(left: Box<dyn Evaluable<f64>>, right: Box<dyn Evaluable<f64>>) -> Self {
        Self { left, right }
    }
}

impl Evaluable<f64> for AddOperator {
    fn eval(&self) -> f64 {
        self.left.eval() + self.right.eval()
    }

    fn print(&self) -> String {
        self.left.print() + " + " + self.right.print().as_str()
    }
}

#[derive(Debug)]
pub struct SubtractOperator {
    left: Box<dyn Evaluable<f64>>,
    right: Box<dyn Evaluable<f64>>,
}

impl SubtractOperator {
    pub fn new(left: Box<dyn Evaluable<f64>>, right: Box<dyn Evaluable<f64>>) -> Self {
        Self { left, right }
    }
}

impl Evaluable<f64> for SubtractOperator {
    fn eval(&self) -> f64 {
        self.left.eval() - self.right.eval()
    }

    fn print(&self) -> String {
        self.left.print() + " - " + self.right.print().as_str()
    }
}

#[derive(Debug)]
pub struct MultiplyOperator {
    left: Box<dyn Evaluable<f64>>,
    right: Box<dyn Evaluable<f64>>,
}

impl MultiplyOperator {
    pub fn new(left: Box<dyn Evaluable<f64>>, right: Box<dyn Evaluable<f64>>) -> Self {
        Self { left, right }
    }
}

impl Evaluable<f64> for MultiplyOperator {
    fn eval(&self) -> f64 {
        self.left.eval() * self.right.eval()
    }

    fn print(&self) -> String {
        self.left.print() + " * " + self.right.print().as_str()
    }
}

#[derive(Debug)]
pub struct DivideOperator {
    left: Box<dyn Evaluable<f64>>,
    right: Box<dyn Evaluable<f64>>,
}

impl DivideOperator {
    pub fn new(left: Box<dyn Evaluable<f64>>, right: Box<dyn Evaluable<f64>>) -> Self {
        Self { left, right }
    }
}

impl Evaluable<f64> for DivideOperator {
    fn eval(&self) -> f64 {
        self.left.eval() / self.right.eval()
    }

    fn print(&self) -> String {
        self.left.print() + " / " + self.right.print().as_str()
    }
}