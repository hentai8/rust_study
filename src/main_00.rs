mod redis_local;
mod error_test;
mod main_0;
mod user;
mod random;

use std::{
    collections::{HashMap, VecDeque},
    fs::create_dir_all,
    sync::{atomic::AtomicBool, Arc, Mutex},
    // time::{Duration, Instant},
};
use std::alloc::System;
use std::future::Future;
use time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::AsyncReadExt;
use crate::redis_local::RedisLocal;
use user::get_worker_id;
use random::rng_local;

#[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
async fn main() {
    // let a = "a".to_string();
    // let b = "b".to_string();
    // let ab = a + &*b;
    // println!("{}", ab);
    //
    // let login = "huang93.hmc".to_string();
    // for item in login.split(".") {
    //     println!("{}", item);
    // }
    //
    // let items: Vec<&str> = login.split(".").collect();
    // println!("{:#?}", items);
    //
    // let mut x = 1;
    // println!("{}", x);
    // println!("{}", x);
    // let t = x;
    // let tt = x;


    // let xxx = "huanghuang93".to_string();
    // let result0 = isUserExisted(xxx).await;
    // println!("{:#?}", result0);
    // // call()
    //
    // let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
    // assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
    let answer = get_worker_id("hentai8.123".to_string());
    println!("{}", answer);

    let mut redis_instance = redis_local::RedisLocal::init();
    redis_instance.do_something().expect("redis error");;
    error_test::error_test(3);

    // 获取时间戳
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string();
    println!("{:#?}", now);
    rng_local();
}

// fn get_worker_id(login: String) -> String{
//     let digest = md5::compute(login);
//     let hash_string: String = format!("{:X}", digest);
//     let hash_string14 = hash_string[0..13].to_string();
//     println!("hash_string: {}", hash_string14.clone());
//     println!("digest.to_ascii_lowercase(): {:?}", digest.to_vec());
//     // println!("try_from: {:?}", <&[u8;4]>::try_from(hash_string14.as_bytes()));
//     let buf = [0, 0, 0, 1];
//     let num = u32::from_be_bytes(buf);
//     // let back_int =i32::from_be_bytes(hash_string14.as_bytes());
//     // // let back_int = digest.clone().parse::<i32>().unwrap();
//     // println!("back_int: {:#?}", back_int);
//     "123".to_string()
// }

// async fn call() -> String {
//     let result0 = isUserExisted("xxx".to_string()).await;
//     // println!("{:#?}", result0);
//     result0
// }

async fn isUserExisted(user: String) -> String {
    let mut url :String = "http://dx.ichains.vip/api/".to_string();
    let path :String = "user/".to_string();
    url += &*path;
    url += &*user;
    println!("{}", url);
    let resp = reqwest::get(url)
        .await.unwrap()
        .text()
        // .json::<String>()
        // .json::<HashMap<String, String>>()
        .await.unwrap();
    println!("{:#?}", resp);
    if resp == "false" {
        println!("123");
    }
    // println!("{}", resp["origin"]);
    // Ok(())
    resp
}


// fn main() {
//     println!("123");
//     let k = "k3".to_string();
//     let v = "v3334".to_string();
//     // let mut r_conn = redis_local::RedisLocal::redis_connect();
//     // redis_local::RedisLocal::do_something(&mut r_conn).expect("redis error");
//
//     // 原子安全的写法
//     let mut arc = Arc::new(Mutex::new(redis_local::RedisLocal::init()));
//     arc.lock().unwrap().set(k, v).expect("panic message");
//
//
//     let k1 = "k11".to_string();
//     let v1 = "v112".to_string();
//     let mut redis_instance = redis_local::RedisLocal::init();
//     redis_instance.set(k1, v1).expect("panic message");
//
//     let k1 = "k11".to_string();
//     let v1_result = redis_instance.get(k1);
//     print!("v1_result :{}", v1_result);
//
//     let k1 = "k11".to_string();
//     let v1 = "v112".to_string();
//     redis_instance.setnx(k1, v1).expect("panic message");
//
//
// }