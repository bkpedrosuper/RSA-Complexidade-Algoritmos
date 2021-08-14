
extern crate rsa;

use rug::Integer;
use rsa::utils;
use rsa::prime_generator;

fn main() {

    let res = 2 * 3 % 2;
    println!("{}", res);

    let res = utils::fast_exp(Integer::from(2), Integer::from(100), &Integer::from(1000000007));
    
    println!("{}", res);
    // match res {
    //     None => println!("Não existe"),
    //     Some(n) => println!("inv mod = {}", n)
    // }

    println!("{}", prime_generator::get_prime());

}
