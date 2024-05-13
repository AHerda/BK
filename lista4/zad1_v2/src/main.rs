use num_bigint::BigInt;
use std::collections::HashMap;
use std::io::{self};

use zad1_v2::{calculate_private, generate_rsa, is_prime};

fn main() {
    let p: BigInt;
    let q: BigInt;
    let n: BigInt;
    let mut sk_a: HashMap<String, BigInt> = HashMap::new();
    let mut pk_a: HashMap<String, BigInt> = HashMap::new();
    let mut sk_b: HashMap<String, BigInt> = HashMap::new();
    let mut pk_b: HashMap<String, BigInt> = HashMap::new();
    let private_result: (BigInt, BigInt);

    println!("Enter two positive prime numbers p and q:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<&str> = input.trim().split_whitespace().collect();
    p = nums[0].parse().unwrap();
    q = nums[1].parse().unwrap();

    println!("Checking if p and q prime (only for small numbers) and positive");
    if p < BigInt::ZERO || q < BigInt::ZERO {
        println!("p and q must be positive!");
        return;
    }
    else if !is_prime(p.clone()) || !is_prime(q.clone()) { // uncomment only for small numbers
        println!("p and q must be prime!");
        return;
    } 
    else {
        println!("\t Confirmed");
    }

    n = p.clone() * q.clone();

    println!("Generating RSA for person A");
    let (e_a, d_a) = generate_rsa(p.clone(), q.clone());
    pk_a.insert("n".to_owned(), n.clone());
    pk_a.insert("e".to_owned(), e_a);
    sk_a.insert("n".to_owned(), n.clone());
    sk_a.insert("d".to_owned(), d_a);

    println!("Generating RSA for person B");
    let (e_b, d_b) = generate_rsa(p.clone(), q.clone());
    pk_b.insert("n".to_owned(), n.clone());
    pk_b.insert("e".to_owned(), e_b);
    sk_b.insert("n".to_owned(), n.clone());
    sk_b.insert("d".to_owned(), d_b);

    println!("Calculating private key for person A");
    private_result = calculate_private(
        n,
        (*pk_a.get("e").unwrap()).clone(),
        (*sk_a.get("d").unwrap()).clone(),
    );

    println!(
        "Calculated p: {} Calculated q: {}",
        private_result.0, private_result.1
    );
}
