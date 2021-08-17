
use rug::Integer;
use super::prime_generator;
use super::utils;

#[derive(Debug)]
pub struct KeyGenerator {
    public_key1: Integer,
    public_key2: Integer,
    private_key: Integer,
}

impl KeyGenerator {
    pub fn new(size: u32) -> KeyGenerator {
        let mut public_key1;
        let mut public_key2;
        let mut private_key;
        'outer: loop {
            let p = prime_generator::get_prime(size/2);
            let q = prime_generator::get_prime(size/2);
            let totn = Integer::from(&p - 1) * Integer::from(&q - 1);
            public_key1 = Integer::from(p * q);
            
            public_key2 = prime_generator::get_prime_max(&public_key1);

            let res = utils::inv_mod(&public_key2, &totn);

            match res {
                Some(n) => private_key = n,
                None => continue 'outer,
            }

            if (Integer::from(&public_key2 * &private_key) % totn) == 1 {
                break;
            }
        }
        KeyGenerator {
            public_key1: public_key1,
            public_key2: public_key2,
            private_key: private_key,
        }
    }

    pub fn get_public_key1(&self) -> &Integer {
        &self.public_key1
    }

    pub fn get_public_key2(&self) -> &Integer {
        &self.public_key2
    }

    pub fn get_private_key(&self) -> &Integer {
        &self.private_key
    }
}