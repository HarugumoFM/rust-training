use std::str::FromStr;
use std::env;


fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn exec_gcd() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common dividsor of {:?} is {}", numbers, d);
}

#[allow(dead_code)]
fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            let temp = m;
            m = n;
            n = temp;
        }
        m = m%n;
    }
    n
}

