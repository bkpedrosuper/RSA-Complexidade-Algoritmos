
use std::time::{SystemTime, UNIX_EPOCH};
use rug::Integer;
use rug::rand::RandState;
use rsa::utils;

#[allow(dead_code)]
pub fn pollar_rho_1(pub_key: &Integer) -> Option<(Integer, Integer)> {
    println!("Breaking the key: {}", pub_key);

    if pub_key == &Integer::from(1) {
        // No prime divisor for 1
        return None;
    }
    if Integer::from(pub_key % 2) == 0 {
        // Numero par significa que 2 é um dos divisores
        return Some((Integer::from(pub_key / 2), Integer::from(2)));
    }

    let mut rand = RandState::new();
    let seed = Integer::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros());
    rand.seed(&seed);

    let mut x = Integer::from(pub_key.random_below_ref(&mut rand));
    let mut y = x.clone();
    let c = Integer::from(pub_key.random_below_ref(&mut rand));
    
    let two = Integer::from(2);
    
    let mut d = Integer::from(1);
    
    while &d == &Integer::from(1) {
        x = (utils::fast_exp(&x, &two, pub_key) + &c + pub_key) % pub_key;
        y = (utils::fast_exp(&y, &two, pub_key) + &c + pub_key) % pub_key;
        y = (utils::fast_exp(&y, &two, pub_key) + &c + pub_key) % pub_key;
        d = Integer::from((Integer::from(&x-&y).abs()).gcd_ref(pub_key));

        // Se n conseguir encontrar tenta com outros números randomicos
        if d == *pub_key { return pollar_rho_1(pub_key) }
    }

    Some((Integer::from(pub_key/&d), d))
}

pub fn pollar_rho_2(pub_key: &Integer) -> Option<(Integer, Integer)> {
    println!("Breaking the key: {}", pub_key);

    let mut i = Integer::from(1);
    let mut k = Integer::from(2);
    let mut d;
    let mut x = Integer::from(3);
    let mut y = Integer::from(3);

    loop {
        x = Integer::from(&x * &x) % pub_key;
        x += Integer::from(pub_key - 1);
        x %= pub_key;
        d = Integer::from(&x - &y).abs().gcd(pub_key);
        if &d != &Integer::from(1) && &d != pub_key {
            return Some((Integer::from(pub_key/&d), d));
        }
        if i == k {
            y = x.clone();
            k <<= 1;
        }
        i += 1;
    }

}
