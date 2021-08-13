
// extern crate RSA;

mod utils;
mod prime_generator;

fn main() {

    let res = utils::inv_mod(3, 26);
    match res {
        None => println!("Não existe"),
        Some(n) => println!("inv mod = {}", n)
    }

    println!("{}", prime_generator::get_prime());

}
