fn main() {
    // 不可变引用
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 可变引用
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    // 同一作用域，特定数据只能有一个可变引用：
    let r1 = &mut s;
    // let r2 = &mut s;

    // 很多时候，大括号可以帮我们解决一些编译不通过的问题，通过手动限制变量的作用域：
    let mut s = String::from("hello");

    {
        let r1 = &mut s;

    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;

    // 可变引用与不可变引用不能同时存在

}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

//
// 修复错误
// fn main() {
//     let mut s = String::from("hello, ");
//
//     push_str(&mut s)
// }
//
// fn push_str(s: &mut String) {
//     s.push_str("world")
// }
