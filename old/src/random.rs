use rand::{thread_rng, Rng};
use hex::encode;

pub fn rng_local() -> String {
    let mut xxx = thread_rng();
    let value = [0, 0, 0, 0];
    let value_ref = value.as_ref();

    let mut sum = Vec::new();
    for byte in value_ref {
        let random_number = xxx.gen_range(0..=255);
        let new_byte = byte + random_number;
        sum.push(new_byte);
    }
    let s = hex::encode(sum.clone());
    s
}