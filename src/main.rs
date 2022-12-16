mod redis_local;
mod error_test;
mod main_1;
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
use redis::cluster::ClusterConnection;



pub struct Server0 {
    pub(crate) redis_connection: ClusterConnection,
}



#[tokio::main]
async fn main() {
    println!("123");
    let k = "k3".to_string();
    let v = "v3334".to_string();
    // let mut r_conn = redis_local::RedisLocal::redis_connect();
    // redis_local::RedisLocal::do_something(&mut r_conn).expect("redis error");

    let mut red = RedisLocal::init();
    red.set(k, v).await.expect("TODO: panic message");

    // // 原子安全的写法
    // let mut arc = Arc::new(Mutex::new(redis_local::RedisLocal::init()));
    // arc.lock().unwrap().set(k, v).expect("panic message");
    //
    //
    // let k1 = "k11".to_string();
    // let v1 = "v112".to_string();
    // let mut redis_instance = redis_local::RedisLocal::init();
    // redis_instance.set(k1, v1).expect("panic message");
    //
    // let k1 = "k11".to_string();
    // let v1_result = redis_instance.get(k1);
    // print!("v1_result :{}", v1_result);
    //
    // let k1 = "k11".to_string();
    // let v1 = "v112".to_string();
    // redis_instance.setnx(k1, v1).expect("panic message");


}

pub async fn isUserExisted(user: String) -> String {
    let mut url :String = "https://www.dxpool.net/api/".to_string();
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

