use crate::girls::azra::hello_zaya;
use crate::girls::alex::hello_ace;

// declare submodules
// the compiler will look for the submodule's code w/in the directory named for the parent module
mod azra;
mod alex;

pub fn hello_girls() {
    let zaya_age = hello_zaya();
    let ace_age = hello_ace();
    println!("zaya_age={zaya_age} and ace_age={ace_age}");
}
