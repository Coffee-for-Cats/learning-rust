fn main() {
    let mut x = 5;
    println!("x is: {x}");

    x = x + 1;
    {
        let x = x * 2;
        println!("x is: {x}");
    }
    println!("x is: {x}");
}
