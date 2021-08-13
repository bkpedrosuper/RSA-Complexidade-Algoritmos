

pub fn inv_mod(a: i64, m: i64) -> Option<i64> {
    let gcd = gcd(a, m);
    if gcd.g != 1 {
        return None
    } 
    Some((gcd.x % m + m) % m) 
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
