
fn main() {
    for i in 1..=5 {
        println!("{}", i);
    }
    // for item in &mut collection {
    //     // ...
    // }

    let a = [4, 3, 2, 1];
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
    // loop {
    //     println!("again!");
    // }
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // 这只会跳出 inner1 循环
                break 'inner1; // 这里使用 `break` 也是一样的
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }

            continue 'outer;
        }
    }

    assert!(count == 30)
}
