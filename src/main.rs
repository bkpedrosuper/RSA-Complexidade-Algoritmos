
extern crate rsa;
extern crate clap;
mod breaker;

use std::time::SystemTime;
use std::fs;
use rsa::encoder;
use rsa::key_generator;
use rsa::key_generator::KeyGenerator;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = config().get_matches();

    let mut key_size = 64;
    if matches.occurrences_of("key_size") != 0 {
        key_size = matches.value_of("key_size").expect("key_size must have a valid value")
            .parse::<u32>().expect("key_size must be an integer");
    }

    let mut times = 1;
    if matches.occurrences_of("times") != 0 {
        times = matches.value_of("times").expect("times must have a valid value")
            .parse::<u32>().expect("times must be an integer");
    }

    let mut b = false;
    if let Some(_matches) = matches.subcommand_matches("break") {
        b = true;
    }
    
    for _ in 0..times {
        run(key_size, b);
    }

}

fn config() -> App<'static, 'static> {
    App::new("RSA")
        .version("0.1")
        .about("RSA encoder and decoder")
        .arg(Arg::with_name("times")
            .short("t")
            .long("times")
            .default_value("1")
            .help("How much times to run"))
        .arg(Arg::with_name("key_size")
            .short("k")
            .long("key_size")
            .default_value("64"))
        .arg(Arg::with_name("output_path")
                        .short("o")
                        .long("output")
                        .help("Output path to put executio times")
                        .default_value("output")
                        .index(1))
        .subcommand(SubCommand::with_name("break")
                        .about("Break the keys"))
}

fn run(key_size: u32, _break: bool) {

    let contents = fs::read_to_string("example.in").expect("Something went wrong while opening the file");

    let start = SystemTime::now();
    let keys = key_generator::KeyGenerator::new(key_size);
    let end = SystemTime::now();

    println!("Key generation time: {:?}", end.duration_since(start).unwrap());

    let start = SystemTime::now();
    let enc = encoder::encrypt(&contents, keys.get_public_key2(), keys.get_public_key1());
    let end = SystemTime::now();
    
    println!("Encryption time: {:?}", end.duration_since(start).unwrap());
    
    fs::write("encrypted", enc.clone()).expect("Something went wrong while");

    // println!("encrypted message:\n {}", enc);

    let start = SystemTime::now();
    let dec = encoder::decrypt(&enc, keys.get_private_key(), keys.get_public_key1());
    let end = SystemTime::now();

    println!("Decryption time: {:?}", end.duration_since(start).unwrap());

    assert_eq!(contents, dec);

    fs::write("decrypted", dec).expect("Something went wrong while opening the file");

    if _break {
        bench_breaking(keys);
    }

}

fn bench_breaking(keys: KeyGenerator) {

    let start = SystemTime::now();

    let (_p, _q) = breaker::pollar_rho_2(keys.get_public_key1());

    let end = SystemTime::now();

    println!("Breaking key time: {:?}", end.duration_since(start).unwrap())

}
