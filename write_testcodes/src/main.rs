pub fn add(x:i32, y:i32) -> i32 {
    return x + y
}

#[test]
fn test_add() {
    assert_eq!(0, add(0,0));
    assert_eq!(10, add(4,6));
    assert_eq!(20, add(15,5));
}

#[test]
#[should_panic]
fn test_panic() {
    panic!("expected panic");
}

fn main() {
    println!("Hello, world!");
}
