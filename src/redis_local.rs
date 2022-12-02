// use std::ops::Add;
// use redis::Commands;
// use redis::Connection;
// use std::string;
// use anyhow::Result;
//
// pub struct RedisLocal {
//     redis_connection:  redis::Connection,
// }
//
// impl RedisLocal {
//     pub fn init() -> RedisLocal {
//         let redis_connection = redis::Client::open("redis://127.0.0.1/")
//             .expect("Invalid connection URL")
//             .get_connection()
//             .expect("failed to connect to Redis");
//         RedisLocal { redis_connection }
//     }
//
//     pub fn do_something(&mut self) -> anyhow::Result<()> {
//         let _: () = self.redis_connection.set("{stratum}:aleo"+"something", "yes").unwrap();
//         println!("Success connect to redis");
//         Ok(())
//     }
//
//     pub fn set(&mut self, key: String, value: String) -> anyhow::Result<()> {
//         key = key.add("{stratum}:aleo:")
//         let _: () = self.redis_connection.set("{stratum}:aleo".to_string()+key, value).unwrap();
//         Ok(())
//     }
//
//     pub fn setnx(&mut self, key: String, value: String) -> anyhow::Result<()> {
//         let _: () = self.redis_connection.set_nx("{stratum}:aleo"+key, value).unwrap();
//         // println!("setnx result: {:#?}", result);
//         Ok(())
//     }
//
//     // pub fn hget(key: String) -> String {
//     //     let mut r_conn = RedisLocal::redis_connect();
//     //     let result: String = r_conn.hget(key).unwrap();
//     //     result
//     // }
//
//     pub fn get(&mut self, key: String) -> String {
//         let result: String = self.redis_connection.get("{stratum}:aleo"+key).unwrap();
//         result
//     }
// }