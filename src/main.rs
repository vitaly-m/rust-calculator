use calculator::str_to_int;

fn main() {
    let parsed = str_to_int("123");
    assert_eq!(123, parsed);
    println!("Hello, world!");
}