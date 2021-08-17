
use rug::Integer;
use super::utils;

pub fn encode(message: &Vec<Integer>, key_1: &Integer, key_2: &Integer) -> Vec<Integer> {
    message.to_vec().iter().map(|v| utils::fast_exp(v, key_1, key_2)).collect()
}

pub fn encrypt(message: String, pub_key1: &Integer, pub_key2: &Integer) -> Vec<String> {
    let v: Vec<Integer> = message.chars().map(|c| char_to_int(c)).collect();
    encode(&v, pub_key1, pub_key2).iter().map(|v| v.to_string_radix(36)).collect()
}

pub fn decrypt(message: &Vec<String>, priv_key: &Integer, pub_key: &Integer) -> String {
     
    let res = encode(message, priv_key, pub_key);
    let res: Vec<char> = res.iter().map(|v| int_to_char(v)).collect();
    res.into_iter().collect()
}

pub fn char_to_int(c: char) -> Integer {
    Integer::from(c as u32)
}

pub fn int_to_char(i: &Integer) -> char {
    let res = i.to_u8().unwrap();
    res as char
}
