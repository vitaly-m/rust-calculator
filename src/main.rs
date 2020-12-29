use std::str::FromStr;
use std::{env, process};

use calculator::{Evaluable, EvaluableResult, Expression};

fn main() {
    let mut args = env::args();
    args.next();
    let ex = args.next().unwrap_or_else(|| {
        eprintln!("Type an expression without spaces as an argument, e.g. \"2+2/(5-3)\"");
        process::exit(1);
    });
    let expression = Expression::from_str(&ex).unwrap_or_else(|err| {
        eprintln!("Problems parsing expression: {}", err);
        process::exit(1);
    });

    match expression.eval() {
        EvaluableResult::F64(v) => println!("{} = {}", ex, v),
        EvaluableResult::Bool(v) => println!("{} = {}", ex, v),
    }
}
