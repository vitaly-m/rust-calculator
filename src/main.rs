use std::convert::TryFrom;

fn main() {
    let parsed = str_to_int("123");
    assert_eq!(123, parsed);
    println!("Hello, world!");
}

fn str_to_int(s: &str) -> i64 {
    let mut res: i64 = 0;
    let mut pow = u32::try_from(s.len()).expect("too long string")-1;
    for c in s.chars() {
        let d = c.to_digit(10).expect("not a digit");
        res += i64::from(d) * 10_i64.pow(pow);
        if pow > 0 {
            pow -= 1;
        }
    }
    return res;
}

#[test]
fn it_works() {
    assert_eq!(123, str_to_int("123"));
    assert_eq!(0, str_to_int("0"));
}