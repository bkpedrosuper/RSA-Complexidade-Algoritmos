

use rug::Integer;

pub fn check_composite(n: &Integer, a: &Integer, d: &Integer, s: u128) -> bool {
    let mut x = fast_exp(a, d, n);
    if x == 1 || x == Integer::from(n - 1) {
        return false;
    }
    for _ in 1..s {
        x = Integer::from(&x * &x) % n;
        if x == Integer::from(n - 1) {
            return false;
        }
    }
    true
}

pub fn fast_exp(base: &Integer, e: &Integer, m: &Integer) -> Integer {
    let zero: Integer = Integer::from(0);
    let one: Integer = Integer::from(1);

    let mut base = base.clone();
    let mut e = e.clone();

    let mut res: Integer = Integer::from(1);
    base %= m;
    while e != 0 {
        if Integer::from(&e & &one) != zero {
            res = (res * &base) % m;
        }
        base = Integer::from(&base * &base) % m;
        e /= Integer::from(2);
    }
    res
}

pub fn inv_mod(a: &Integer, m: &Integer) -> Option<Integer> {
    let one: Integer = Integer::from(1);
    let gcd = gcd(&a, &m);
    if gcd.g != one {
        return None;
    }
    Some((gcd.x % m + m) % m)
}

pub fn gcd(a: &Integer, b: &Integer) -> GcdResult {
    if *b == 0 {
        return GcdResult::new(&Integer::from(1), &Integer::from(0), a);
    }
    let res = gcd(b, &Integer::from(a % b));
    GcdResult::new(&res.y, &(res.x - &res.y * Integer::from(a / b)), &res.g)
}

#[derive(Debug)]
pub struct GcdResult {
    x: Integer,
    y: Integer,
    g: Integer,
}

impl GcdResult {
    pub fn new(x: &Integer, y: &Integer, g: &Integer) -> GcdResult {
        GcdResult{ 
            x: x.clone(),
            y: y.clone(),
            g: g.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse_modulus_test() {
        assert_eq!(inv_mod(&Integer::from(3), &Integer::from(26)), Some(Integer::from(9)));

        assert_eq!(inv_mod(&Integer::from(6), &Integer::from(24)), None);

        assert_eq!(inv_mod(&Integer::from(1593), &Integer::from(4265)), Some(Integer::from(1087)));
    }

    #[test]
    fn fast_exp_test() {
        assert_eq!(fast_exp(&Integer::from(2), &Integer::from(100), &Integer::from(1000000007)), Integer::from(976371285));
    }
}
