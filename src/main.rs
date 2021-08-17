
extern crate rsa;

use rsa::key_generator;
use rsa::encoder;
use std::fs;

fn main() {

    let contents = fs::read_to_string("entrada").expect("Something went wrong while opening the file");

    let key_generator = key_generator::KeyGenerator::new(256);
    println!("{:?}", key_generator);

    let enc = encoder::encrypt(contents, key_generator.get_public_key2(), key_generator.get_public_key1());
    
    fs::write("encriptado", format!("{:?}", enc.clone()));

    let dec = encoder::decrypt(&enc, key_generator.get_private_key(), key_generator.get_public_key1());
    println!("{}", dec);

    fs::write("decriptado", dec).expect("Something went wrong while opening the file");

}
