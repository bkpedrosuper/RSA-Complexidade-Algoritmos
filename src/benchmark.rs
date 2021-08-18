
use std::fs;
use std::error::Error;
use std::fs::OpenOptions;
use std::time::SystemTime;
use rsa::{key_generator, encoder};
use std::io::prelude::*;
use super::breaker;

pub fn benchmark(times: u32) -> Result<(), Box<dyn Error>> {
    prepare_paths();
    
    let input = fs::read_to_string("example.in")?;
    
    for i in (32..100).step_by(2) {
        run_times(&input, i, times)?;
    }
    Ok(())
}

pub fn run_times(input: &String, key_size: u32, times: u32) -> Result<(), Box<dyn Error>> {
    
    println!("------------- Key Size {} ------------", key_size);

    let file_options = OpenOptions::new()
                            .create(true)
                            .append(true).clone();
    
    let mut encryption_file = file_options.open(format!("./output/encryption/{}", key_size))?;
    let mut decryption_file = file_options.open(format!("./output/decryption/{}", key_size))?;
    let mut generation_file = file_options.open(format!("./output/generation/{}", key_size))?;
    let mut breaking_file = file_options.open(format!("./output/breaking/{}", key_size))?;

    encryption_file.write_all(b"fodase")?;
    encryption_file.write_all(b"fodase")?;
    encryption_file.write_all(b"fodase")?;

    for _ in 0..times {

        let start = SystemTime::now();
        let keys = key_generator::KeyGenerator::new(key_size);
        let end = SystemTime::now();
        
        generation_file.write_all(format!("{:?}\n", end.duration_since(start).unwrap().as_millis()).as_bytes())?;

        // println!("Key generation time: {:?}", end.duration_since(start).unwrap());
        
        let start = SystemTime::now();
        let enc = encoder::encrypt(input, keys.get_public_key2(), keys.get_public_key1());
        let end = SystemTime::now();
        
        encryption_file.write_all(format!("{:?}\n", end.duration_since(start).unwrap().as_millis()).as_bytes())?;
        
        // println!("Encryption time: {:?}", end.duration_since(start).unwrap());
        
        // fs::write("encrypted", enc.clone())?;
        
        // println!("encrypted message:\n {}", enc);

        let start = SystemTime::now();
        let dec = encoder::decrypt(&enc, keys.get_private_key(), keys.get_public_key1());
        let end = SystemTime::now();

        decryption_file.write_all(format!("{:?}\n", end.duration_since(start).unwrap().as_millis()).as_bytes())?;

        // println!("Decryption time: {:?}", end.duration_since(start).unwrap());
                                    
        assert_eq!(*input, dec);
        
        // fs::write("decrypted", dec).expect("Something went wrong while opening the file");
        
        let start = SystemTime::now();
        
        let (_p, _q) = breaker::pollar_rho_2(keys.get_public_key1());
        
        let end = SystemTime::now();
        
        breaking_file.write_all(format!("{:?}\n", end.duration_since(start).unwrap().as_millis()).as_bytes())?;

        // println!("Breaking key time: {:?}", end.duration_since(start).unwrap());
    
    }

    Ok(())
}

fn prepare_paths() {
    fs::create_dir_all("./output/generation").expect("Unable to create the path");
    fs::create_dir_all("./output/encryption").expect("Unable to create the path");
    fs::create_dir_all("./output/decryption").expect("Unable to create the path");
    fs::create_dir_all("./output/breaking").expect("Unable to create the path");

    for entry in fs::read_dir("./output/generation").unwrap() {
        fs::remove_file(entry.unwrap().path()).expect("error deleting one file");
    }

    for entry in fs::read_dir("./output/encryption").unwrap() {
        fs::remove_file(entry.unwrap().path()).expect("error deleting one file");
    }

    for entry in fs::read_dir("./output/decryption").unwrap() {
        fs::remove_file(entry.unwrap().path()).expect("error deleting one file");
    }

    for entry in fs::read_dir("./output/breaking").unwrap() {
        fs::remove_file(entry.unwrap().path()).expect("error deleting one file");
    }
}
