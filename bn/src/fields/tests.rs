use rand::{SeedableRng, StdRng};
use super::FieldElement;

fn can_invert<F: FieldElement>() {
    let mut a = F::one();

    for _ in 0..10000 {
        assert_eq!(a * a.inverse().unwrap(), F::one());

        a = a + F::one();
    }

    a = -F::one();
    for _ in 0..10000 {
        assert_eq!(a * a.inverse().unwrap(), F::one());

        a = a - F::one();
    }

    assert_eq!(F::zero().inverse(), None);
}

fn rand_element_eval<F: FieldElement>() {
    for _ in 0..100 {
        let a = F::random();
        let b = F::random();
        let c = F::random();
        let d = F::random();

        assert_eq!((a + b) * (c + d), (a * c) + (b * c) + (a * d) + (b * d));
    }
}

fn rand_element_squaring<F: FieldElement>() {
    for _ in 0..100 {
        let a = F::random();

        assert!(a * a == a.squared());
    }

    let mut cur = F::zero();
    for _ in 0..100 {
        assert_eq!(cur.squared(), cur * cur);

        cur = cur + F::one();
    }
}

fn rand_element_addition_and_negation<F: FieldElement>() {
    for _ in 0..100 {
        let a = F::random();

        assert_eq!(a + (-a), F::zero());
    }

    for _ in 0..100 {
        let mut a = F::random();
        let r = F::random();
        let mut b = a + r;

        for _ in 0..10 {
            let r = F::random();
            a = a + r;
            b = b + r;

            let r = F::random();
            a = a - r;
            b = b - r;

            let r = F::random();
            a = a + (-(-r));
            b = b + (-(-r));

            let r = F::random();
            a = a - r;
            b = b + (-r);

            let r = F::random();
            a = a + (-r);
            b = b - r;
        }

        b = b - r;
        assert_eq!(a, b);
    }
}

fn rand_element_inverse<F: FieldElement>() {
    for _ in 0..10000 {
        let a = F::random();
        assert!(a.inverse().unwrap() * a == F::one());
        let b = F::random();
        assert_eq!((a * b) * (a.inverse().unwrap()), b);
    }
}

fn rand_element_multiplication<F: FieldElement>() {
    // If field is not associative under multiplication, 1/8 of all triplets a, b, c
    // will fail the test (a*b)*c = a*(b*c).

    for _ in 0..250 {
        let a = F::random();
        let b = F::random();
        let c = F::random();

        assert_eq!((a * b) * c, a * (b * c));
    }
}

pub fn field_trials<F: FieldElement>() {
    can_invert::<F>();

    assert_eq!(-F::zero(), F::zero());
    assert_eq!(-F::one() + F::one(), F::zero());
    assert_eq!(F::zero() - F::zero(), F::zero());

    //let seed: [usize; 4] = [103245, 191922, 1293, 192103];
    //let mut rng = StdRng::from_seed(&seed);

    rand_element_squaring::<F>();
    rand_element_addition_and_negation::<F>();
    rand_element_multiplication::<F>();
    rand_element_inverse::<F>();
    rand_element_eval::<F>();
}
