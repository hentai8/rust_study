use std::fmt::Debug;

// 这里T是泛型
// 函数 report 接受一个泛型参数 T，它必须实现 Debug trait。Debug trait 用于在调试输出中以调试格式打印数据。
// 通过使用泛型参数和 trait 约束，函数 report 可以接受不同类型的参数，并在输出中使用 Debug trait 来打印它们。
fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}

fn clear(text: &mut String) -> () {
    *text = String::from("");
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}

fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    let z = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
}

fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // 表达式
    };
    println!("x: {}, y: {}", x, y);
    let z = add_with_extra(x, y);
    println!("z: {}", z);
    let a = 8;
    // vec是一个动态数组
    // f64是float64的简写
    let b: Vec<f64> = Vec::new();
    let (a, c) = ("hi", false);
    report(1);
    assert_eq!(ret_unit_type(), ());
}