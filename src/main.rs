use std::borrow::Borrow;
use std::sync::Arc;

struct MyStruct {
    job_pools: Vec<String>,
}

impl MyStruct {
    fn add_string(&mut self, string: String) {
        self.job_pools.push(string);
    }
}

fn main() {
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
