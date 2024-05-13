use std::collections::HashMap;
use std::io::{self};

use zad1_v2::{generate_rsa, is_prime, calculate_private};

fn main() {
    let a: i128;
    let b: i128;
    let p: i128;
    let q: i128;
    let n: i128;
    let mut sk_a: HashMap<String, i128> = HashMap::new();
    let mut pk_a: HashMap<String, i128> = HashMap::new();
    let mut sk_b: HashMap<String, i128> = HashMap::new();
    let mut pk_b: HashMap<String, i128> = HashMap::new();
    let private_result: (i128, i128);

    println!("Enter two positive prime numbers p and q:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<&str> = input.trim().split_whitespace().collect();
    a = nums[0].parse().unwrap();
    b = nums[1].parse().unwrap();
    p = a as i128;
    q = b as i128;

    println!("{} {}", a, b);
    println!("{} {}", p, q);
    println!("Checking if p and q prime");
    if a < 0 || b < 0 {
        println!("p and q must be positive!");
        return;
    } else if !is_prime(p) || !is_prime(q) {
        println!("p and q must be prime!");
        return;
    } else {
        println!("\t Confirmed");
    }

    n = p * q;

    println!("Generating RSA for person A");
    let (e_a, d_a) = generate_rsa(p, q);
    pk_a.insert("n".to_owned(), n);
    pk_a.insert("e".to_owned(), e_a);
    sk_a.insert("n".to_owned(), n);
    sk_a.insert("d".to_owned(), d_a);

    println!("Generating RSA for person B");
    let (e_b, d_b) = generate_rsa(p, q);
    pk_b.insert("n".to_owned(), n);
    pk_b.insert("e".to_owned(), e_b);
    sk_b.insert("n".to_owned(), n);
    sk_b.insert("d".to_owned(), d_b);

    println!("Calculating private key for person A");
    private_result = calculate_private(n, *pk_a.get("e").unwrap(), *sk_a.get("d").unwrap());

    println!(
        "Calculated p: {} Calculated q: {}",
        private_result.0, private_result.1
    );
}
