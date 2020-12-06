#[test]
fn it_adds_two() {
    assert_eq!(123, calculator::str_to_int("123"));
    assert_eq!(0, calculator::str_to_int("0"));
}