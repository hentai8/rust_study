// 元组 tuple
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}