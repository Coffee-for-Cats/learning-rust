use std::io;

const ERROR_MESSAGE: &str =
    "Something went wrong! Try removing characters and putting just C or F as input!";

fn main() {
    println!("Converting calculator in Rust!");

    println!("Type the number you wanna convert, and then a C (celcius) or a F (fahrenheit):");
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect(ERROR_MESSAGE);

    //Gets the scale. It can be a C or F.
    let mut scale: char = temperature
        .chars()
        //3, because 1 is the 0x00, other is a \r, and it's an array that starts with 0.
        .nth(temperature.chars().count() - 3)
        .expect(ERROR_MESSAGE);
    scale.make_ascii_uppercase();

    //Should get the temperature in a number, without the last letter.
    let temperature: String = {
        let mut chars = temperature.chars();
        //I have to skip the last 3 characters, same reason as line 18~
        for _i in 1..4 {
            chars.next_back();
        }
        let chars: &str = chars.as_str();
        chars.to_string() //returning String type
    };
    let temperature: f64 = temperature.trim().parse().expect(ERROR_MESSAGE);

    //print the results
    println!(
        "The temperature in {} is {}",
        {
            if scale == 'F' {
                "celsius"
            } else if scale == 'C' {
                "fahrenheit"
            } else {
                panic!("{ERROR_MESSAGE}");
            }
        },
        {
            if scale == 'F' {
                from_f_to_c(temperature)
            } else if scale == 'C' {
                from_c_to_f(temperature)
            } else {
                panic!("{ERROR_MESSAGE}");
            }
        }
    );
}

fn from_f_to_c(f: f64) -> f64 {
    return (f - 32.0) * 5.0 / 9.0;
}

fn from_c_to_f(c: f64) -> f64 {
    return c * 9.0 / 5.0 + 32.0;
}
