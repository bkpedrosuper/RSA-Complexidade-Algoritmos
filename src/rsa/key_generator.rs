
use rug::Integer;
use super::prime_generator;
use super::utils;

#[derive(Debug)]
pub struct KeyGenerator {
    public_key1: Integer,
    public_key2: Integer,
    private_key: Integer,
    q: Integer,
    p: Integer,
    totn: Integer,
}

impl KeyGenerator {
    pub fn new(size: u32) -> KeyGenerator {
        let mut public_key1;
        let mut public_key2;
        let mut private_key;
        let mut p;
        let mut q;
        let mut totn;
        'outer: loop {
            p = prime_generator::get_prime(size/2);
            q = prime_generator::get_prime(size/2);
            totn = Integer::from(&p - 1) * Integer::from(&q - 1);
            println!("{}\n{}", p, q);
            public_key1 = Integer::from(&p * &q);
            
            public_key2 = prime_generator::get_prime_max(&public_key1);

            let res = utils::inv_mod(&public_key2, &totn);

            match res {
                Some(n) => private_key = n,
                None => continue 'outer,
            }

            if (Integer::from(&public_key2 * &private_key) % &totn) == 1 {
                break;
            }

        }
        KeyGenerator {
            public_key1: public_key1,
            public_key2: public_key2,
            private_key: private_key,
            q: q,
            p: p,
            totn: totn,
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::utils;

    #[test]
    fn encryption_256_test() {
        let key_generator = KeyGenerator::new(256);

        let enc = utils::fast_exp(&Integer::from(7), key_generator.get_public_key2(), key_generator.get_public_key1());
        let dec = utils::fast_exp(&enc, key_generator.get_private_key(), key_generator.get_public_key1());

        assert_eq!(7, dec);
    }

    #[test]
    fn encryption_512_test() {
        let key_generator = KeyGenerator::new(512);

        let enc = utils::fast_exp(&Integer::from(77), key_generator.get_public_key2(), key_generator.get_public_key1());
        let dec = utils::fast_exp(&enc, key_generator.get_private_key(), key_generator.get_public_key1());

        assert_eq!(77, dec);
    }

    #[test]
    fn encryption_1024_test() {
        let key_generator = KeyGenerator::new(1024);

        let enc = utils::fast_exp(&Integer::from(777), key_generator.get_public_key2(), key_generator.get_public_key1());
        let dec = utils::fast_exp(&enc, key_generator.get_private_key(), key_generator.get_public_key1());

        assert_eq!(777, dec);
    }
}
