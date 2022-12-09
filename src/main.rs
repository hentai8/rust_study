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
    println!("The map has {} entries.", *COUNT);
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
    println!("A expensive calculation on a static results in: {}.", *NUMBER);
    JOBPOOLS.lock().unwrap().push("nnn".to_string());
    println!("{:#?}", JOBPOOLS.lock().unwrap());

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
