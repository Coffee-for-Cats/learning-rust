use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");
    //println!("The secret number is {secret_number}");

    loop {
        println!("Please, input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //O .cmp() retorna um Result, que pode ser Ordering::Less, Greater ou Equal (são tipos, que são parte do std::cmp::Ordering), e o match compara esse Result com essas três "arms", sendo elas "Less", "Greater", ou "Equal", e executa uma Arrow Function na primeira de fechar.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!();
    }
}
