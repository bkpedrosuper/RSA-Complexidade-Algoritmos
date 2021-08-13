
use rug::Integer;

const ZERO: Integer = Integer::from(0);
const ONE: Integer = Integer::from(1);

pub fn fast_exp(mut a: Integer, mut e: Integer, m: Integer) -> Integer {
    let mut res: Integer = ONE;
    a %= m.clone();
    while e >= ONE {
        let res = e & ONE;
        if res == ZERO {
            res = m.clone() + res * a % m.clone();
        }
        a = a.clone() * a.clone() % m;
        e >>= 1;
    }
    res
}

pub fn  inv_mod(a: Integer, m: Integer) -> Option<Integer> {
    let gcd = gcd(&a, &m);
    match gcd.g {
        ONE => Some((gcd.x % m.clone() + m.clone()) % m),
        _ => None
    }
}

pub fn gcd(a: &Integer, b: &Integer) -> GcdResult {
    let zero: Integer = Integer::from(0);
    let um: Integer = Integer::from(1);
    if b.clone() == zero {
        return GcdResult::new(&um, &zero, a);
    }
    let res = gcd(b, &Integer::from(a % b));
    GcdResult::new(&res.y.clone(), &Integer::from(res.x-res.y*Integer::from(a/b)), &res.g)
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
        assert_eq!(inv_mod(Integer::from(3), Integer::from(26)), Some(Integer::from(9)));

        assert_eq!(inv_mod(Integer::from(6), Integer::from(24)), None);

        assert_eq!(inv_mod(Integer::from(1593), Integer::from(4265)), Some(Integer::from(1087)));
    }


}
