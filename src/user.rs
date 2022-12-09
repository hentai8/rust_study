use crypto::digest::Digest;
use crypto::md5::Md5;
use num::bigint::BigInt;
use num::Num;

pub fn get_worker_id(s: String)-> String {
    let mut m = Md5::new();
    m.input_str(&s);
    let temp = m.result_str();
    let temp =temp[0..13].to_string();
    let n = BigInt::from_str_radix(&temp,16).unwrap();
    let result = n + BigInt::from(10 as u64).pow(17);
    result.to_str_radix(10)
}