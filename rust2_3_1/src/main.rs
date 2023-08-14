fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    // 不可变的
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}", x, y);

    // 可变的
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // 但这段代码似乎与我们刚刚学到的内容相矛盾：没有调用 clone，不过依然实现了类似深拷贝的效果 —— 没有报所有权的错误。
    //
    // 原因是像整型这样的基本类型在编译时是已知大小的，会被存储在栈上，所以拷贝其实际的值是快速的。
    // 这意味着没有理由在创建变量 y 后使 x 无效（x、y 都仍然有效）。
    // 换句话说，这里没有深浅拷贝的区别，因此这里调用 clone 并不会与通常的浅拷贝有什么不同，我们可以不用管它（可以理解成在栈上做了深拷贝）。
    //
    // Rust 有一个叫做 Copy 的特征，可以用在类似整型这样在栈中存储的类型。
    // 如果一个类型拥有 Copy 特征，一个旧的变量在被赋值给其他变量后仍然可用。

    // 那么什么类型是可 Copy 的呢？可以查看给定类型的文档来确认，不过作为一个通用的规则：
    // 任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的。如下是一些 Copy 的类型：
    let x = 5;
    let y = x;
    let x = 10;
    println!("x = {}, y = {}", x, y);

    let s = String::from("fuck");
    takes_ownership(s);
    // 会报错，因为不是基本类型
    // println!("{}", s);

    let z = 5;
    makes_copy(z);
    println!("{}", z);

    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // 但是，这里 `age` 变量却是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    //println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);
}

// 因为不是基本类型
// 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放
fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

// 把s return出去，就不会被销毁了，调用的时候也拿一个变量去接一下
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

// 因为是基本类型
// 这里，some_integer 移出作用域。不会有特殊操作
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}