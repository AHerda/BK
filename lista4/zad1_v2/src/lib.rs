use rand::Rng;
use rayon::iter::FromParallelIterator; // Add this line
use rayon::iter::ParallelIterator; // Add this line
use rayon::prelude::*;

fn gcd_extended(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        return (b, 0, 1);
    }

    let (gcd, x1, y1) = gcd_extended(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;

    (gcd, x, y)
}

fn modular_inverse(e: i128, k: i128) -> Option<i128> {
    let (gcd, x, _) = gcd_extended(e, k);
    // Modular inverse does not exist
    if gcd != 1 {
        return None;
    }

    // Ensure d is positive
    Some((x % k + k) % k)
}

pub fn is_prime(number: i128) -> bool {
    if number == 2 || number == 3 {
        return true
    }
    if number <= 1 || number % 2 == 0 || number % 3 == 0 {
        return false
    }

    (5..)
        .step_by(6)
        .take_while(|i| i * i <= number)
        .all(|i| number % i != 0 && number % (i + 2) != 0)
}

pub fn generate_rsa(p: i128, q: i128) -> (i128, i128) {
    let phi = (p - 1) * (q - 1);

    let mut rng = rand::thread_rng();
    let mut e_candidate = rng.gen_range(2..phi);

    while gcd_extended(e_candidate, phi).0 != 1 {
        e_candidate = rng.gen_range(2..phi);
    }

    (e_candidate, modular_inverse(e_candidate, phi).unwrap())
}

// fn mod_pow2( base: i128, exponent: i128, modulus: i128 ) -> i128 {
//     (0..exponent).into_par_iter().reduce(|| 1, |acc, _| (acc * base) % modulus)
// }

fn mod_pow(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp >>= 1;
    }
    result
}

pub fn calculate_private(n: i128, e: i128, d: i128) -> (i128, i128) {
    let phi = d * e - 1;
    let mut t = phi;
    let mut a = 2;
    let mut k;
    let mut x;
    let mut p = 0;
    let q;

    while t % 2 == 0 {
        t >>= 1;
    }

    while a < 100 {
        //println!("a: {}", a);
        k = t;
        while k < phi {
            //println!("k: {}, phi: {}", k, phi);

            x = mod_pow(a, k, n);
            //println!("x: {}", x);

            if x != 1 && x != (n - 1) && mod_pow(x, 2, n) == 1 {
                p = gcd_extended(x - 1, n).0;
                break;
            }
            k *= 2;
        }
        a += 2;
    }
    q = n / p;

    (p, q)
}
