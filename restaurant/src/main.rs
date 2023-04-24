//This is a bin (binary) file/path/module.
//I have to have one in my crate ("restaurant") to compile and execute it!

//Here, I use the library in this crate (with the same name as the folder)
//to access the functions inside them.
use restaurant::eat_at_restaurant;

fn main() {
    eat_at_restaurant();
}
