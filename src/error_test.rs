pub fn error_test(number: i32) {
    println!("The number is {}", number);
    match number {
        1 => println!("one"),
        2 => {
            println!("two");
        }
        _ => {
            println!("err")
        }
    }
}
