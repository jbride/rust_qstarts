use std::{fs, io::{self, Read}};

const file_path: &str = "/etc/resolv.conf";

fn read_file_super_shorthand() -> Result<String, io::Error> {
    let f_content = fs::read_to_string(file_path)?;
    Ok(f_content)
}

fn read_file_shorthand() -> Result<String, io::Error> {
    let mut f_content = String::new();
    std::fs::File::open(file_path)?.read_to_string(&mut f_content)?;
    Ok(f_content)
}

fn main() {
    match read_file_super_shorthand() {
        Ok(f) => println!("{}", f),
        Err(e) => panic!("error path = {},\texception = {}", file_path, e)
    }
    match read_file_shorthand() {
        Ok(f) => println!("{}", f),
        Err(e) => panic!("error path = {},\texception = {}", file_path, e)
    }

}
