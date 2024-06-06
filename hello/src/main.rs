#![warn(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

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



// specify a short-cut to the hello_girls function of the 'girls' module
use crate::girls::hello_girls;

// in this crate root file, reference the 'girls' module
mod girls;

fn main() {
    println!("Hello, world!");
    let f_int = 23;
    let s_int = 54;
    let gcd_result = gcd(f_int,s_int);
    println!("main gcd_result of {f_int},{s_int} = {gcd_result}");
    hello_girls();
}

