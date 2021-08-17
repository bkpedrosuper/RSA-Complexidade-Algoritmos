
extern crate rsa;

// use std::env;
use rsa::key_generator;
use rsa::encoder;
use std::fs;
mod breaker;
use std::time::SystemTime;

fn main() {
    // let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string("example.in").expect("Something went wrong while opening the file");

    let key_generator = key_generator::KeyGenerator::new(100);

    let enc = encoder::encrypt(&contents, key_generator.get_public_key2(), key_generator.get_public_key1());
    
    fs::write("encrypted", enc.clone()).expect("Something went wrong while");

    let dec = encoder::decrypt(&enc, key_generator.get_private_key(), key_generator.get_public_key1());
    println!("{}", dec);

    fs::write("decrypted", dec).expect("Something went wrong while opening the file");

    let start = SystemTime::now();

    match breaker::pollar_rho_2(key_generator.get_public_key1()) {
        None => println!("It was not possible to break the key"),
        Some((p, q)) => println!("q = {}\np = {}", q, p),
    }

    let end = SystemTime::now();

    println!("Breaking key time: {:?}", end.duration_since(start).unwrap())

}
