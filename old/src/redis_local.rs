use redis::Commands;
use redis::Connection;
use redis::cluster::ClusterConnection;
use redis::Pipeline;
use redis::cluster::ClusterClient;
use std::string;
use redis::ConnectionLike;
use anyhow::Result;

pub struct RedisLocal {
    pub(crate) redis_connection: ClusterConnection,
}

impl RedisLocal {
    pub fn init() -> RedisLocal {
        let node = vec!["redis://:chainsAb@@43.206.117.127:7001"];
        let client = ClusterClient::new(node).unwrap();
        let redis_connection = client.get_connection().unwrap();
        RedisLocal { redis_connection }
    }

    pub async fn do_something(&mut self) -> anyhow::Result<()> {
        let _: () = self.redis_connection.set("{stratum}:aleo:something", "yes").unwrap();
        println!("Success connect to redis");
        Ok(())
    }

    pub async fn set(&mut self, key: String, value: String) -> anyhow::Result<()> {
        let mut key0 = "{stratum}:aleo:".to_string();
        key0 += &*key;
        let _: () = self.redis_connection.set(key0, value).unwrap();
        Ok(())
    }
    pub async fn hset(&mut self, key: String, field: String, value: String) -> anyhow::Result<()> {
        let mut key0 = "{stratum}:aleo:".to_string();
        key0 += &*key;
        let _: () = self.redis_connection.hset(key0, field, value).unwrap();
        Ok(())
    }


    pub async fn setnx(&mut self, key: String, value: String) -> anyhow::Result<()> {
        let mut key0 = "{stratum}:aleo:".to_string();
        key0 += &*key;
        let _: () = self.redis_connection.set_nx(key0, value).unwrap();
        // println!("setnx result: {:#?}", result);
        Ok(())
    }

    // pub fn hget(key: String) -> String {
    //     let mut r_conn = RedisLocal::redis_connect();
    //     let result: String = r_conn.hget(key).unwrap();
    //     result
    // }

    pub async fn get(&mut self, key: String) -> String {
        let mut key0 = "{stratum}:aleo:".to_string();
        key0 += &*key;
        let result: String = self.redis_connection.get(key0).unwrap();
        result
    }
    pub async fn hincr(&mut self, key: String, field: String, increment: i64) -> anyhow::Result<()> {
        let mut key0 = "{stratum}:aleo:".to_string();
        key0 += &*key;
        let _: () = self.redis_connection.hincr(key0, field, increment).unwrap();
        // result
        Ok(())
    }
}