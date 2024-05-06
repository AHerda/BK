use rand::Rng;
use std::collections::HashMap;
use rayon;

fn gcd_extended(a: u64, b: u64) -> (u64, u64, u64) {
    if a == 0 {
        return (b, 0, 1);
    }

    let (gcd, x1, y1) = gcd_extended(b % a, a);

    let x = y1 - (b / a) * x1;
    let y = x1;

    (gcd, x, y)
}

fn modular_inverse(e: u64, k: u64) -> u64 {
    let (gcd, x, _) = gcd_extended(e, k);

    if gcd != 1 {
        // Modular inverse does not exist
        return -1;
    } else {
        // Ensure d is positive
        let d = ((x % (k as i64)) + (k as i64)) % (k as i64);
        return d;
    }
}

fn is_prime(number: u64) -> bool {
    if number == 2 || number == 3 {
        return true
    }
    if number <= 1 || number % 2 == 0 || number % 3 == 0 {
        return false
    }
}