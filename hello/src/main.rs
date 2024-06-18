use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::env;

// specify a short-cut to the hello_girls function of the 'girls' module
use crate::girls::hello_girls;

// in this crate root file, reference the 'girls' module
mod girls;

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

// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors
fn read_file_long_form(file_path: &String) -> Result<String, std::io::Error> {
    let file_content_result = File::open(file_path);
    let mut resolv_file = match file_content_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut file_content = String::new();
    match resolv_file.read_to_string(&mut file_content) {
        Ok(_) => Ok(file_content),
        Err(e) => Err(e),
    }
    
}

// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
fn read_file_shortcut(file_path: &String) -> Result<String, std::io::Error> {
    let mut file_content = String::new();
    File::open(file_path)?.read_to_string(&mut file_content)?;
    Ok(file_content)
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

    let file_path = String::from("/etc/resolv.conf");
    let file_content = match read_file_long_form(&file_path) {
        Ok(f) => f,
        Err(e) => panic!("Error opening file {} \nerror = {}", file_path, e)
    };
    println!("file content long format = \n\n{}", file_content);

    let file_content_shortcut = match read_file_shortcut(&file_path) {
        Ok(f) => f,
        Err(e) => panic!("Error opening file with shortcut{} \nerror = {}", file_path, e)
    };
    println!("file content shortcut = \n\n{}", file_content_shortcut);
}
