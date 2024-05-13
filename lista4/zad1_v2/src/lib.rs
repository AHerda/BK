// use rand::Rng;
// use rayon::iter::FromParallelIterator; // Add this line
// use rayon::iter::ParallelIterator; // Add this line
use num_bigint::{BigInt, RandBigInt};

fn gcd_extended(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if a == BigInt::ZERO {
        return (b, BigInt::ZERO, BigInt::from(1));
    }

    let (gcd, x1, y1) = gcd_extended(b.clone() % a.clone(), a.clone());
    let x = y1 - (b / a) * x1.clone();
    let y = x1;

    (gcd, x, y)
}

fn modular_inverse(e: BigInt, k: BigInt) -> Option<BigInt> {
    let (gcd, x, _) = gcd_extended(e, k.clone());
    // Modular inverse does not exist
    if gcd != BigInt::from(1) {
        return None;
    }

    // Ensure d is positive
    Some((x % k.clone() + k.clone()) % k)
}

pub fn is_prime(number: BigInt) -> bool {
    if number == BigInt::from(2) || number == BigInt::from(3) {
        return true;
    }
    if number <= BigInt::from(1)
        || number.clone() % 2 == BigInt::ZERO
        || number.clone() % 3 == BigInt::ZERO
    {
        return false;
    }

    let mut i = BigInt::from(5);
    while i.clone() * i.clone() <= number.clone() {
        if number.clone() % i.clone() == BigInt::from(0)
            || number.clone() % (i.clone() + 2) == BigInt::from(0)
        {
            return false;
        }
        i += BigInt::from(6);
    }

    return true;
}

pub fn generate_rsa(p: BigInt, q: BigInt) -> (BigInt, BigInt) {
    let phi: BigInt = (p - 1) * (q - 1);

    let mut rng = rand::thread_rng();
    let mut e_candidate = rng.gen_bigint_range(&BigInt::from(2), &phi);

    while gcd_extended(e_candidate.clone(), phi.clone()).0 != BigInt::from(1) {
        e_candidate = rng.gen_bigint_range(&BigInt::from(2), &phi);
    }

    (
        e_candidate.clone(),
        modular_inverse(e_candidate.clone(), phi).unwrap(),
    )
}

// fn mod_pow2( base: BigInt, exponent: BigInt, modulus: BigInt ) -> BigInt {
//     (0..exponent).into_par_iter().reduce(|| 1, |acc, _| (acc * base) % modulus)
// }

fn mod_pow(mut base: BigInt, mut exp: BigInt, modulus: BigInt) -> BigInt {
    let mut result = BigInt::from(1);
    base %= modulus.clone();
    while exp > BigInt::ZERO {
        if exp.clone() % 2 == BigInt::from(1) {
            result = (result * base.clone()) % modulus.clone();
        }
        base = (base.clone() * base) % modulus.clone();
        exp >>= 1;
    }
    result
}

pub fn calculate_private(n: BigInt, e: BigInt, d: BigInt) -> (BigInt, BigInt) {
    let phi = d * e - BigInt::from(1);
    let mut t = phi.clone();
    let mut a = BigInt::from(2);
    let mut k;
    let mut x;
    let mut p = BigInt::from(0);
    let q;

    while t.clone() % 2 == BigInt::ZERO {
        t >>= 1;
    }

    while a < BigInt::from(100) {
        //println!("a: {}", a);
        k = t.clone();
        while k < phi {
            //println!("k: {}, phi: {}", k, phi);

            x = mod_pow(a.clone(), k.clone(), n.clone());
            //println!("x: {}", x);

            if x != BigInt::from(1)
                && x != (n.clone() - 1)
                && mod_pow(x.clone(), BigInt::from(2), n.clone()) == BigInt::from(1)
            {
                p = gcd_extended(x - 1, n.clone()).0;
                break;
            }
            k *= 2;
        }
        a += 2;
    }
    q = n / p.clone();

    (p, q)
}
