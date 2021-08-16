
use rug::Integer;
use super::prime_generator;
use super::utils;

pub struct KeyGenerator {
    public_key1: Integer,
    public_key2: Integer,
    private_key: Integer,
}

impl KeyGenerator {
    pub fn new(size: u32) -> KeyGenerator {
        let mut public_key1 = Integer::new();
        let mut public_key2 = Integer::new();
        let mut private_key = Integer::new();
        while {
            let p = prime_generator::get_prime(size/2);
            let q = prime_generator::get_prime(size/2);
            let totn = Integer::from(&p - 1) * Integer::from(&q - 1);
            public_key1 = Integer::from(p * q);
            while {
                public_key2 = prime_generator::get_prime(size);
            
                public_key2 >= totn
            } {}

            let res = utils::gcd(&public_key2, &totn);

            private_key = (res.x() % &totn + &totn) % &totn;

            (public_key2 * private_key % totn) != 1
        } {}
        KeyGenerator {
            public_key1: public_key1,
            public_key2: public_key2,
            private_key: private_key,
        }
    }

    pub fn get_public_key1(self) -> Integer {
        self.public_key1
    }

    pub fn get_public_key2(self) -> Integer {
        self.public_key2
    }

    pub fn get_private_key(self) -> Integer {
        self.private_key
    }
}
