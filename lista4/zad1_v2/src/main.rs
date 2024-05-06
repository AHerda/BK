use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    let mut a: i64;
    let mut b: i64;
    let mut p: u64;
    let mut q: u64;
    let mut d_a: u64;
    let mut e_a: u64;
    let mut d_b: u64;
    let mut e_b: u64;
    let mut n: u64;
    let mut sk_a: HashMap<String, u64> = HashMap::new();
    let mut pk_a: HashMap<String, u64> = HashMap::new();
    let mut sk_b: HashMap<String, u64> = HashMap::new();
    let mut pk_b: HashMap<String, u64> = HashMap::new();
    let mut private_result: (u64, u64);

    println!("Enter two positive prime numbers p and q:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<&str> = input.trim().split_whitespace().collect();
    a = nums[0].parse().unwrap();
    b = nums[1].parse().unwrap();
    p = a as u64;
    q = b as u64;

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
    generate_rsa(p, q, &mut e_a, &mut d_a);
    pk_a.insert("n".to_owned(), n);
    pk_a.insert("e".to_owned(), e_a);
    sk_a.insert("n".to_owned(), n);
    sk_a.insert("d".to_owned(), d_a);

    println!("Generating RSA for person B");
    generate_rsa(p, q, &mut e_b, &mut d_b);
    pk_b.insert("n".to_owned(), n);
    pk_b.insert("e".to_owned(), e_b);
    sk_b.insert("n".to_owned(), n);
    sk_b.insert("d".to_owned(), d_b);

    private_result = calculate_private(n, *pk_a.get("e").unwrap(), *sk_a.get("d").unwrap());

    println!("Calculated p: {} Calculated q: {}", private_result.0, private_result.1);
}

fn is_prime(n: u64) -> bool {
    // Implement this function
    unimplemented!()
}

fn generate_rsa(p: u64, q: u64, e: &mut u64, d: &mut u64) {
    // Implement this function
    unimplemented!()
}

fn calculate_private(n: u64, e: u64, d: u64) -> (u64, u64) {
    // Implement this function
    unimplemented!()
}