fn main() {
    another_function(5, 6.1);
    let x = plus_five(5);

    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: f64) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_five(x: i32) -> i32 {
    x + 5
}

fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}

fn clear(text: &mut string) -> () {
    *text = string::from("");
}

fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
}

fn forever() -> ! {
    loop {
        //...
    };
}