/// Paillier cryptosystem
mod millerrabin;
use bn::arith::U256;
use num_traits::{Zero, One};
use hope::protocol::model::*;

pub struct Paillier {
    pub pk: PaillierEncryptionKey,
    pub sk: Option<PaillierDecryptionKey>,
}

impl Paillier {
    pub fn new(keysize: usize) -> Paillier {
        let (pk, sk) = Paillier::keygen(keysize);
        Paillier {
            pk: pk,
            sk: Some(sk),
        }
    }

    pub fn keygen(keysize: usize) -> (PaillierEncryptionKey, PaillierDecryptionKey) {
        assert!(keysize % 2 == 0);
        let p = millerrabin::generate_strong_prime(keysize / 2)
            .to_bigint()
            .unwrap();
        let q = millerrabin::generate_strong_prime(keysize / 2)
            .to_bigint()
            .unwrap();
        let n: BigInt = &p * &q;
        let g = &n + One::one();
        let lambda: BigInt = (&p - One::one()) * (&q - One::one());
        let mu = lambda.invert(&n).unwrap();
        let n2 = &n * &n;
        (
            PaillierEncryptionKey { n: n, n2: n2, g: g },
            PaillierDecryptionKey {
                lambda: lambda,
                mu: mu,
            },
        )
    }

    pub fn encrypt(ek: &PaillierEncryptionKey, m: &BigInt) -> BigInt {
        let mut r = millerrabin::generate_urandom_inrange(Zero::zero(), ek.n.to_biguint().unwrap())
            .to_bigint()
            .unwrap();
        while r.gcd(&ek.n) != One::one() {
            r = millerrabin::generate_urandom_inrange(Zero::zero(), ek.n.to_biguint().unwrap())
                .to_bigint()
                .unwrap();
        }
        let rn = r.modpow(&ek.n, &ek.n2);
        let gm = m * &ek.n + One::one(); // faster version
        // let gm = self.pk.g.powm(m, &self.pk.n2);
        (&gm * &rn) % &ek.n2
    }

    pub fn decrypt(dk: &PaillierDecryptionKey, ek: &PaillierEncryptionKey, c: &BigInt) -> BigInt {
        let cl: BigInt = c.modpow(&dk.lambda, &ek.n2);
        let lc: BigInt = (cl - One::one()) as &BigInt / &ek.n;
        (&lc * &dk.mu) % &ek.n
    }

    pub fn rerandomize(ek: &PaillierEncryptionKey, m: &BigInt) -> BigInt {
        let r = millerrabin::generate_urandom_inrange(Zero::zero(), ek.n.to_biguint().unwrap())
            .to_bigint()
            .unwrap();
        let rn = r.modpow(&ek.n, &ek.n2);
        (m as &BigInt * rn) % &ek.n2
    }

    pub fn add(ek: &PaillierEncryptionKey, c1: &BigInt, c2: &BigInt) -> BigInt {
        (c1 * c2) % &ek.n2
    }

    pub fn sub(ek: &PaillierEncryptionKey, c1: &BigInt, c2: &BigInt) -> Option<BigInt> {
        match Paillier::mult_inv(ek, c2) {
            Some(inv) => Some((c1 * inv) % &ek.n2),
            None => None,
        }
    }

    pub fn mult_inv(ek: &PaillierEncryptionKey, c1: &BigInt) -> Option<BigInt> {
        let a: BigInt = c1 % &ek.n2;
        let mut x: BigInt = One::one();
        while x < ek.n2 {
            if (a * x) % &ek.n2 == One::one() {
                Some(x);
            }
            x = x + One::one();
        }
        None
    }

    pub fn add_const(ek: &PaillierEncryptionKey, c: &BigInt, m: &BigInt) -> BigInt {
        Paillier::add(ek, c, &ek.g.modpow(&m, &ek.n2))
    }

    pub fn mul_const(ek: &PaillierEncryptionKey, c: &BigInt, m: &BigInt) -> BigInt {
        c.modpow(&m, &ek.n2)
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn paillier_add() {
        let p = Paillier::new(256);
        let n1 = p.encrypt(100);
        let n2 = p.encrypt(101);
        assert_eq!(p.decrypt(p.encrypt(201)), p.decrypt(p.add_cipher(n1, n2)));
    }
}
