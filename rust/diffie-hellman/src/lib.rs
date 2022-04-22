use modexp::modexp;
use num::ToPrimitive;

pub fn private_key(p: u64) -> u64 {
    p - 1
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modexp(g, a, p).to_u64().unwrap()
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modexp(b_pub, a, p).to_u64().unwrap()
}
