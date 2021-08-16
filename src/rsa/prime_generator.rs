
use std::time::{SystemTime, UNIX_EPOCH};
use rug::Integer;
use rug::rand::{RandState};
use rug::Assign;
use super::utils;

pub fn get_prime(size: u32) -> Integer {
    let mut rand = RandState::new();
    let seed = Integer::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros());
    rand.seed(&seed);
    let mut i = Integer::new();
    while !miller_rabin(i.clone()) {
        i.assign(Integer::random_bits(size, &mut rand));
    }
    i
}

pub fn miller_rabin(n: Integer) -> bool {
    if n < 4 {
        return n == 2 || n == 3;
    }
    
    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    let primes: Vec<Integer> = primes.iter().map(|&x| Integer::from(x)).collect();

    let mut s: u128 = 0;
    let mut d = Integer::from(&n - 1);
    while Integer::from(&d & 1) == 0 {
        d >>= 1; 
        s += 1;
    }
    for a in primes {
        if a == n.clone() {
            return true;
        }
        if utils::check_composite(&n, Integer::from(a), d.clone(), s) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_prime() {
        assert!(miller_rabin(Integer::from(2)));
        assert!(miller_rabin(Integer::from(1000000007)));
    }

    #[test]
    fn shouldnt_be_prime() {
        assert!(!miller_rabin(Integer::from(6)));
        assert!(!miller_rabin(Integer::from(2000000009)));    
    }

    #[test]
    fn two_get_primes_shouldnt_be_eq() {
        assert_ne!(get_prime(128), get_prime(128));
    }

}

