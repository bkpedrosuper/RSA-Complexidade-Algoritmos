
extern crate rsa;
mod breaker;

use std::env;
use std::time::SystemTime;
use std::fs;
use rsa::encoder;
use rsa::key_generator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 && args[1] == "break" {
        let mut key_size = 64;
        if args.len() == 3 {
            key_size = args[2].parse::<u32>().unwrap();
        }
        bench_breaking(key_size);
    } else {
        run();
    }
    
}

fn run() {

    let contents = fs::read_to_string("example.in").expect("Something went wrong while opening the file");

    let keys = key_generator::KeyGenerator::new(100);

    let enc = encoder::encrypt(&contents, keys.get_public_key2(), keys.get_public_key1());
    
    fs::write("encrypted", enc.clone()).expect("Something went wrong while");

    println!("encrypted message:\n {}", enc);

    let dec = encoder::decrypt(&enc, keys.get_private_key(), keys.get_public_key1());
    
    println!("decripted message:");
    println!("{}", dec);

    fs::write("decrypted", dec).expect("Something went wrong while opening the file");

}

fn bench_breaking(key_size: u32) {

    let keys = key_generator::KeyGenerator::new(key_size);

    let start = SystemTime::now();

    let (_p, _q) = breaker::pollar_rho_2(keys.get_public_key1());

    let end = SystemTime::now();

    println!("Breaking key time: {:?}", end.duration_since(start).unwrap())
    

}
