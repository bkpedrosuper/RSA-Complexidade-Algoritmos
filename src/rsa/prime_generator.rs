
use rug::Integer;
use super::utils;

pub fn get_prime() -> u64 {
    let a = 7;
    a
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
        assert_eq!(miller_rabin(Integer::from(2)), true);
        assert_eq!(miller_rabin(Integer::from(1000000007)), true);
    }

    #[test]
    fn shouldnt_be_prime() {
        assert_eq!(miller_rabin(Integer::from(6)), false);
        assert_eq!(miller_rabin(Integer::from(2000000009)), false);    
    }

}

