use std::borrow::Borrow;
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
    static ref COUNT: usize = HASHMAP.len();
    static ref NUMBER: u32 = times_two(21);
    static ref JOBPOOLS: Mutex<Vec<String>> = {
        let mut x = Mutex::new(vec!["null".to_string()]);
        x.lock().unwrap().push("111".to_string());
        x
    };
    static ref HEIGHTNOW: Mutex<i32> = {
        let mut x = Mutex::new(10);
        x
    };
}

fn times_two(n: u32) -> u32 { n * 2 }


struct MyStruct {
    job_pools: Vec<String>,
}

impl MyStruct {
    fn add_string(&mut self, string: String) {
        self.job_pools.push(string);
    }
}

fn main() {
    // 定义一个变量
    let mutex = Mutex::new(10);

    // 获取一个 MutexGuard
    let mut guard = mutex.lock().unwrap();

    // 通过 MutexGuard 修改变量的值
    *guard = 100;

    // 使用 guard 访问变量的值
    println!("The value is: {}", *guard);


    println!("The map has {} entries.", *COUNT);
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
    println!("A expensive calculation on a static results in: {}.", *NUMBER);

    println!("{}", HEIGHTNOW.lock().unwrap());
    let mut xxx = HEIGHTNOW.lock().unwrap();
    *xxx = 100;
    println!("{}", *xxx);
    // println!("{}", *HEIGHTNOW.lock().unwrap());


    JOBPOOLS.lock().unwrap().push("000".to_string());
    println!("{:#?}", JOBPOOLS.lock().unwrap());
    if JOBPOOLS.lock().unwrap().contains(&"nnn".to_string()) {
        println!("success");
    } else { println!("false") }

    let mut my_struct = MyStruct {
        job_pools: vec![String::from("hello"), String::from("world")],
    };
    println!("{:#?}", my_struct.job_pools);
    my_struct.add_string("!".to_string());
    println!("{:#?}", my_struct.job_pools);

// let my_struct = Arc::new(MyStruct {
//     job_pools: vec![String::from("hello"), String::from("world")],
// });
// let cloned = my_struct.clone();
// println!("{:#?}", cloned.job_pools);
// my_struct.add_string("!".to_string());
// println!("{:#?}", cloned.job_pools);
}
