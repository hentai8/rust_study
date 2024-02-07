use redis::cluster::{ClusterClient, ClusterConnection};
use redis::Commands;
use lazy_static::lazy_static;
use std::sync::Mutex;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{channel, Sender},
        RwLock,
    },
    task,
};
use std::time::Duration;


lazy_static! {
    static ref REDIS: Mutex<Vec<ClusterConnection>> = {
        let node = vec!["redis://:chainsAb@@43.206.117.127:7001"];
        let client = ClusterClient::new(node).unwrap();
        let redis_connection = client.get_connection().unwrap();
        let mut x = Mutex::new(vec![redis_connection]);
        x
    };
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut puid = "hentai1".to_string();

    let mut ip = "127.0.1.1".to_string();

    let mut worker_id = "333".to_string();

    // 添加黑名单,在这里计数,在prover_submit中判断
    let blacklist_reject_key = "blacklist:reject_user".to_string();
    let blacklist_reject_field = puid.clone() + &*":".to_string() + &*worker_id.clone();
    let mut blacklist_reject_key = "{stratum-0}:aleo:".to_string() + &*blacklist_reject_key;
    let _: () = REDIS.lock().unwrap()[0].hincr(blacklist_reject_key, blacklist_reject_field, 1).unwrap();

    let blacklist_reject_ip_key = "blacklist:reject_ip".to_string();
    let blacklist_reject_ip_field = ip;
    let mut blacklist_reject_ip_key = "{stratum-0}:aleo:".to_string() + &*blacklist_reject_ip_key;
    let _: () = REDIS.lock().unwrap()[0].hincr(blacklist_reject_ip_key, blacklist_reject_ip_field, 1).unwrap();

    let task = tokio::spawn(async move {
        loop {
            // interval.tick().await;


            let keys: Vec<String> = REDIS.lock().unwrap()[0].hkeys("{stratum-0}:aleo:blacklist:reject_user").unwrap();
            for key in keys {
                let value: i32 = REDIS.lock().unwrap()[0].hget("{stratum-0}:aleo:blacklist:reject_user", &key).unwrap();
                if value > 10 {
                    let _: () = REDIS.lock().unwrap()[0].hincr("{stratum-0}:aleo:blacklist:user", &key, 1).unwrap();
                }
                let _: () = REDIS.lock().unwrap()[0].hdel("{stratum-0}:aleo:blacklist:reject_user", &key).unwrap();
            }

            let keys: Vec<String> = REDIS.lock().unwrap()[0].hkeys("{stratum-0}:aleo:blacklist:reject_ip").unwrap();
            for key in keys {
                let value: i32 = REDIS.lock().unwrap()[0].hget("{stratum-0}:aleo:blacklist:reject_ip", &key).unwrap();
                if value > 10 {
                    let _: () = REDIS.lock().unwrap()[0].hincr("{stratum-0}:aleo:blacklist:ip", &key, 1).unwrap();
                }
                let _: () = REDIS.lock().unwrap()[0].hdel("{stratum-0}:aleo:blacklist:reject_ip", &key).unwrap();
            }
            // 每隔5分钟执行一次
            tokio::time::sleep(Duration::from_secs(300)).await;
        }
    });
    let _result = task.await;
}
