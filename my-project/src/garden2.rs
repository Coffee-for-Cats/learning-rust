pub mod garden3;
use garden3::return1;

pub fn hello() {
    if return1() == 1 {
        println!("Hello, Rust!");
    }
}
