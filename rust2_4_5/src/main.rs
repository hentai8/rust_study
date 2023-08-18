fn main() {
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];

    // 错误的
    // let array = [String::from("rust is good!"); 8];
    let array: [String; 8] = std::array::from_fn(|i| String::from("rust is good!"));
    println!("{:?}", array);

    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    // 加上下面这一行后，上面的one 和blank1会自动变更数据类型
    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    for a in &arrays {
        print!("{:?}:", a);

        // 迭代器
        // for n in a{
        //     print!("\t{} + 10 = {}", n, n + 10);
        // }
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
}
