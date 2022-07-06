use std::io;

fn main() {
    let mut number = String::new();

    println!("Input the fibonacci number you want:");
    io::stdin().read_line(&mut number).expect("Error!");

    let number: i32 = number
        .trim()
        .parse()
        .expect("Please, type a valid number only!");

    println!(
        "The Fibonacci number of {} is: {}",
        number,
        fast_fib(number)
    );
}

fn fast_fib(n: i32) -> i32 {
    let mut last2fib = (1, 0);
    for _i in (1..n) {
        let first = last2fib.0;
        let secnd = last2fib.1;

        last2fib = (first + secnd, first);
    }

    return last2fib.0;
}
