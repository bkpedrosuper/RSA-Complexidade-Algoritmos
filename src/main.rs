
extern crate rsa;

use rug::Integer;
use num_bigint::BigInt;
use num_bigint::{ToBigInt};
use rsa::utils;
use rsa::teste;
use rsa::prime_generator;

fn main() {

    // let a: BigInt = 3.to_bigint().unwrap();
    // let b: BigInt = 26.to_bigint().unwrap();
    let res = teste::inv_mod(Integer::from(3), Integer::from(26));
    match res {
        None => println!("Não existe"),
        Some(n) => println!("inv mod = {}", n)
    }

    println!("{}", prime_generator::get_prime());

}
