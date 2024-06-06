
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n!=0 && m!=0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

// compile and run the test code only when executing "cargo test" (not when executing "cargo build")
#[cfg(test)]
mod tests {
    use super::*;

    #[test]         // rust "attribute" .... similar to Java annotation
    fn test_gcd() {
        assert_eq!(gcd(14,15), 1);
    }
}


use std::str::FromStr;
use std::env;

// specify a short-cut to the hello_girls function of the 'girls' module
use crate::girls::hello_girls;

// in this crate root file, reference the 'girls' module
mod girls;

fn main() {
    println!("Hello, world!");

    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing arg"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ....");
        std::process::exit(1);
    }

    let f_int = numbers[0];
    let s_int = numbers[1];
    let gcd_result = gcd(f_int,s_int);
    println!("main gcd_result of {f_int},{s_int} = {gcd_result}");
    hello_girls();
}

