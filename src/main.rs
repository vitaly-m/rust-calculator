use std::str::FromStr;

use calculator::{Evaluable, Expression};

fn main() {
    match Expression::from_str("2+2") {
        Ok(e) => println!("{:?}={:?}", e, e.eval()),
        Err(err) => println!("err {}", err),
    }
}
