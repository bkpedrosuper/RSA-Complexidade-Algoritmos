
extern crate rsa;

// use std::env;
use rsa::key_generator;
use rsa::encoder;
use std::fs;

fn main() {
    // let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string("in").expect("Something went wrong while opening the file");

    let key_generator = key_generator::KeyGenerator::new(256);

    let enc = encoder::encrypt(&contents, key_generator.get_public_key2(), key_generator.get_public_key1());
    
    fs::write("encrypted", enc.clone()).expect("Something went wrong while");

    let dec = encoder::decrypt(&enc, key_generator.get_private_key(), key_generator.get_public_key1());
    println!("{}", dec);

    fs::write("decrypted", dec).expect("Something went wrong while opening the file");

}
