use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Test {
    width: u32,
    height: u32,
}

impl Test {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn wider(&self, test: &Test) -> bool {
        self.width > test.width
    }
}

// 单元结构体
struct UnitStruct;

// 泛型结构体
struct Point<T> {
    x: T,
    y: T,
}

struct Point12<T1, T2> {
    x: T1,
    y: T2,
}

// 泛型func
fn get_last<T>(array: &[T]) -> &T {
    &array[array.len() - 1]
}


// 特性 类似interface 也可以设置默认特性
trait Comparable {
    fn greater(&self, b: &Circle) -> bool;
    fn less(&self, b: &Circle) -> bool;
    // 默认特性
    fn equals(&self, b: &Circle) -> bool {
        true
    }
}

struct Circle {
    radius: f64,
    center: (f64, f64),
}

// 实现特性
impl Comparable for Circle {
    fn greater(&self, b: &Circle) -> bool {
        self.radius > b.radius
    }

    fn less(&self, b: &Circle) -> bool {
        self.radius < b.radius
    }

    fn equals(&self, b: &Circle) -> bool {
        self.radius == b.radius
    }
}

fn sub_thread() {
    for i in 1..5 {
        println!("Sub thread print {}", i);
    }
}

// 静态变量 即 全局变量
static VAR: i32 = 123;

// 属性
// 获取环境变量
#[cfg(target_os = "linux")]
const OS: &str = "Linux";

#[cfg(target_os = "windows")]
const OS: &str = "Windows";

fn main() {
    // 常量
    const CONST_NAME: i32 = 123;
    let x = 3.0; //f64
    let y: f32 = 3.0; //f32
    let mut string1 = String::from("some string");
    string1.push('1');
    string1.push_str("1234");

    // 元组: 元组和列表最大的区别就是，列表中的元素可以进行任意修改，
    // 就好比是用铅笔在纸上写的字，写错了还可以擦除重写；
    // 而元组中的元素无法修改，除非将元组整体替换掉，
    // 就好比是用圆珠笔写的字，写了就擦不掉了，除非换一张纸。
    // 可以理解为，tuple 元组是一个只读版本的list列表。
    let tuple1 = (500, 6.4, 1);
    let tuple2: (i64, f64, i64) = (500, 6.4, 1);
    println!("tuple1.0 is {}", tuple1.0);
    println!("tuple1.1 is {}", tuple1.1);
    println!("tuple1.2 is {}", tuple1.2);

    // 元组结构体
    struct Color(u8, u8, u8);
    let black = Color(0, 0, 0);

    // 数组
    let a = [1, 2, 3, 4, 5];
    let b = ["a", "b", "c"];
    let c: [&str; 2] = ["1", ""];

    //


    let test1 = Test { width: 30, height: 60 };
    let test2 = Test { width: 30, height: 50 };

    let point1 = Point::<i32> { x: 1, y: 2 };
    let point2 = Point::<f64> { x: 1.0, y: 2.0 };
    let point3 = Point { x: 1, y: 2 };
    let point12 = Point12 { x: 1.0, y: 2 };
    println!("Hello, world!");
    println!("test1 is {:#?}", test1);
    println!("test1's area is {}", test1.area());
    println!("{}", test1.wider(&test2));


    let c1 = Circle {
        radius: 10.0,
        center: (0.0, 0.0),
    };
    let c2 = Circle {
        radius: 5.0,
        center: (1.0, 3.0),
    };

    println!("(c1>c2)={}", c1.greater(&c2));
    println!("(c1<c2)={}", c1.less(&c2));
    println!("(c1==c2)={}", c1.equals(&c2));

    // 多线程
    thread::spawn(sub_thread).join().unwrap();
    for i in 1..5 {
        println!("Main thread print {}", i);
    }
    thread::spawn(|| {
        for i in 1..5 {
            println!("Sub thread print {}", i);
            thread::sleep(Duration::from_millis(1))
        }
    });
    for i in 1..5 {
        println!("Main thread print {}", i);
        thread::sleep(Duration::from_millis(2))
    }

    println!("Your system os is {}", OS);

    // 线程通信&互斥锁
}
