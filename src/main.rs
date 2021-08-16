
extern crate rsa;

use rug::Integer;
use rsa::utils;
use rsa::prime_generator;
use rsa::key_generator;

fn main() {

    let res = 2 * 3 % 2;
    println!("{}", res);

    let res = utils::fast_exp(Integer::from(2), Integer::from(100), &Integer::from(1000000007));
    
    println!("{}", res);

    println!("{}", prime_generator::get_prime(128));

    println!("{:?}", key_generator::KeyGenerator::new(128));

}
