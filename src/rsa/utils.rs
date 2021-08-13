
// use num_bigint::{BigInt, BigUint};
// use num_bigint::ToBigInt;
// use num_traits::{One, Zero};

// pub fn fast_exp(a: BigInt, e: BigInt, m: BigInt) -> BigInt {
//     let mut a = &a.clone();
//     let mut e = e.clone();
//     let m = m.clone();

//     let mut res: BigInt = One::one();
//     a %= m.clone();
//     while e >= One::one() {
//         let aaa = e.to_biguint().unwrap();
//         let asdf: BigUint = One::one();
//         let perdemo = aaa & asdf;
//         if perdemo.is_one() {
//             res = m.clone() + res * a % m.clone();
//         }
//         a = a.clone() * a.clone() % m;
//         e >>= 1;
//     }
//     res
// }

// pub fn  inv_mod(a: BigInt, m: BigInt) -> Option<BigInt> {
//     let gcd = gcd(&a, &m);
//     let um: BigInt = One::one();
//     if gcd.g == um {
//         return Some((gcd.x % m.clone() + m.clone()) % m);
//     }
//     None
// }

// pub fn gcd(a: &BigInt, b: &BigInt) -> GcdResult {
//     let zero: BigInt = Zero::zero();
//     let um: BigInt = One::one();
//     if b.clone() == zero {
//         return GcdResult::new(&um, &zero, a);
//     }
//     let res = gcd(b, &(a % b));
//     GcdResult::new(&res.y, &(res.x-res.y.clone()*(a/b)), &res.g)
// }

// #[derive(Debug)]
// pub struct GcdResult {
//     x: BigInt,
//     y: BigInt,
//     g: BigInt,
// }

// impl GcdResult {
//     pub fn new(x: &BigInt, y: &BigInt, g: &BigInt) -> GcdResult {
//         GcdResult{ 
//             x: x.clone(),
//             y: y.clone(),
//             g: g.clone(),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn inverse_modulus_test() {
//         assert_eq!(inv_mod(3.to_bigint().unwrap(), 26.to_bigint().unwrap()), Some(9.to_bigint().unwrap()));

//         assert_eq!(inv_mod(6.to_bigint().unwrap(), 24.to_bigint().unwrap()), None);

//         assert_eq!(inv_mod(1593.to_bigint().unwrap(), 4265.to_bigint().unwrap()), Some(1087.to_bigint().unwrap()));
//     }


// }
