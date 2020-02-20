use num_bigint::{BigInt, BigUint};
use std::num::FromPrimitive;
use std::num::{Zero, One};
use num_traits::identities::Zero;
use num_traits::cast::FromPrimitive;
use num_traits::identities::One;
use std::env;

const NUM_TESTS: usize = 24;
const TRIAL_DIVISORS: [u32; 167] = [
    3,
    5,
    7,
    11,
    13,
    17,
    19,
    23,
    29,
    31,
    37,
    41,
    43,
    47,
    53,
    59,
    61,
    67,
    71,
    73,
    79,
    83,
    89,
    97,
    101,
    103,
    107,
    109,
    113,
    127,
    131,
    137,
    139,
    149,
    151,
    157,
    163,
    167,
    173,
    179,
    181,
    191,
    193,
    197,
    199,
    211,
    223,
    227,
    229,
    233,
    239,
    241,
    251,
    257,
    263,
    269,
    271,
    277,
    281,
    283,
    293,
    307,
    311,
    313,
    317,
    331,
    337,
    347,
    349,
    353,
    359,
    367,
    373,
    379,
    383,
    389,
    397,
    401,
    409,
    419,
    421,
    431,
    433,
    439,
    443,
    449,
    457,
    461,
    463,
    467,
    479,
    487,
    491,
    499,
    503,
    509,
    521,
    523,
    541,
    547,
    557,
    563,
    569,
    571,
    577,
    587,
    593,
    599,
    601,
    607,
    613,
    617,
    619,
    631,
    641,
    643,
    647,
    653,
    659,
    661,
    673,
    677,
    683,
    691,
    701,
    709,
    719,
    727,
    733,
    739,
    743,
    751,
    757,
    761,
    769,
    773,
    787,
    797,
    809,
    811,
    821,
    823,
    827,
    829,
    839,
    853,
    857,
    859,
    863,
    877,
    881,
    883,
    887,
    907,
    911,
    919,
    929,
    937,
    941,
    947,
    953,
    967,
    971,
    977,
    983,
    991,
    997,
];

pub fn generate_urandom(len: usize) -> BigUint {
    let mut buf = vec![0; len];
    let r = getrandom::getrandom(&mut buf[0..len]);
    let a = BigUint::from_bytes_be(&buf);
    assert!(a.bits() + 1 == len);
    a
}

pub fn generate_urandom_inrange(lower: BigUint, upper: BigUint) -> BigUint {
    loop {
        let p: BigUint = generate_urandom((upper.bits() - lower.bits()) / 2);
        if p.bits() + 1 > lower.bits() && p.bits() + 1 < upper.bits() {
            return p;
        }
    }
}


pub fn generate_prime(len: usize) -> BigUint {
    loop {
        let p: BigUint = nextprime(generate_urandom(len));
        if p.bits() + 1 == len {
            return p;
        }
    }
}

pub fn nextprime(nonp: BigUint) -> BigUint {
    loop {
        let a: BigUint = generate_urandom(nonp.bits());
        let p: BigUint = find_prime(a, NUM_TESTS);
        if p.bits() + 1 == nonp.bits() {
            return p;
        }
    }
}
/// Generate a prime p such that p-1 has a large prime factor
pub fn generate_strong_prime(len: usize) -> BigUint {
    // generate a half-size prime pp
    let pp = generate_prime(len / 2);
    let a = generate_urandom(len - len / 2 + 1);

    let mut p: BigUint = &pp * &a + One::one();
    assert!(p.bits() + 1 >= len);
    loop {
        if p.probab_prime_p(40) {
            if p.bits() + 1 == len {
                return p;
            } else {
                return generate_strong_prime(len);
            }
        } else {
            p = &p + &a;
        }
    }
}

fn find_prime(mut n: BigUint, ntests: usize) -> BigUint {
    // If the input is even, it should be made odd.
    if &n % 2u32 == BigUint::zero() {
        n += 1u32;
    }
    let two: BigUint = BigUint::from_u32(2).unwrap();
    while !isprime(&n, &ntests) {
        n += &two;
    }
    n
}

fn isprime(n: &BigUint, ntests: &usize) -> bool {
    for i in TRIAL_DIVISORS.iter() {
        if n % i == BigUint::zero() {
            return n == &(BigUint::from_u32(*i).unwrap());
        }
    }
    let (d, r) = decompose(n);
    let two: BigUint = BigUint::from_u32(2).unwrap();
    for _ in 0..*ntests {
        let a: BigUint = generate_urandom_inrange(two, (n - 2u16));
        if trial_composite(n, &d, &r, &a) {
            return false;
        }
    }
    true
}


fn trial_composite(n: &BigUint, d: &BigUint, r: &usize, a: &BigUint) -> bool {
    let mut x = a.modpow(&d, &n);
    if (x == BigUint::one()) || (x == (n - 1u32)) {
        return false;
    }
    let two = BigUint::from_u32(2).unwrap();
    for i in 0..(r - 1) {
        let e = d * (&two << i);
        x = x * x % n;
        if n - 1u32 == x {
            return false;
        }
    }

    true
}

fn decompose(n: &BigUint) -> (BigUint, usize) {
    // Split number such that
    // n = d*2^r + 1
    let mut d = n - 1u32;
    let mut r: usize = 0;
    while (&d % 2u32).is_zero() {
        r += 1;
        d /= 2u32;
    }
    (d, r)

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn miller_generate() {
        let p = Paillier::new(256);
        let n1 = p.encrypt(100);
        let n2 = p.encrypt(101);
        assert_eq!(p.decrypt(p.encrypt(201)), p.decrypt(p.add_cipher(n1, n2)));
    }
}
