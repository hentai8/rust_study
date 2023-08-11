fn main() {
    let _y = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let (a, mut b): (bool, bool) = (true, false);
    println!("a: {}, b: {}", a, b);
    b = true;
    assert_eq!(a, b);

    let (c, d) = (1, 4);
    assert_eq!([1, 4], [c, d]);
}