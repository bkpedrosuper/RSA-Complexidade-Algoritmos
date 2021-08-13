

pub fn inv_mod(a: i64, m: i64) -> Option<i64> {
    let gcd = gcd(a, m);
    match gcd.g {
        1 => Some((gcd.x % m + m) % m),
        _ => None,
    }
}

pub fn gcd(a: i64, b: i64) -> GcdResult {
    match b {
        0 => GcdResult::new(1, 0, a),
        b => {
            let res = gcd(b, a % b);
            GcdResult::new(res.y, res.x-res.y*(a/b), res.g)
        }
    }
}

#[derive(Debug)]
pub struct GcdResult {
    x: i64,
    y: i64,
    g: i64,
}

impl GcdResult {
    pub fn new(x: i64, y: i64, g: i64) -> GcdResult {
        GcdResult{ 
            x: x,
            y: y,
            g: g,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse_modulus_test() {
        assert_eq!(
            match inv_mod(3, 26) {
                None => -1,
                Some(n) => n,
        }, 9);

        assert_eq!(
            match inv_mod(6, 34) {
                None => -1,
                Some(n) => n,
        }, -1);

        assert_eq!(
            match inv_mod(1593, 4265) {
                None => -1,
                Some(n) => n,
        }, 1087);
    }

}
