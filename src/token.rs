use std::str::FromStr;
use core::fmt::Debug;

pub enum Token {
    Value(String),
    Add,
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
pub struct AddOperator<L, R>
    where L: Evaluable<f64>,
          R: Evaluable<f64> {
    left: L,
    right: R,
}

impl<L, R> AddOperator<L, R>
    where L: Evaluable<f64>,
          R: Evaluable<f64>{
    pub fn new(left: L, right: R) -> Self {
        AddOperator { left, right }
    }
}

impl<L, R> Evaluable<f64> for AddOperator<L, R>
    where L: Evaluable<f64>,
          R: Evaluable<f64>{
    fn eval(&self) -> f64 {
        self.left.eval() + self.right.eval()
    }

    fn print(&self) -> String {
        self.left.print() + " + " + self.right.print().as_str()
    }
}