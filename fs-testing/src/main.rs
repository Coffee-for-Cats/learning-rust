fn main() {
    let file = std::fs::read_to_string("txts/test.txt");
    let string = match file {
        Ok(string) => string,
        Err(e) => {
            eprintln!("Error reading the file!");
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    let mut lines = string.split('\n');

    let first = lines.next().unwrap();
    let first_temp = get_temp(first);
    let mut smallest_temp = first_temp;
    let mut biggest_temp  = first_temp;
    let mut sum = 0.0;
    let mut count = 0;

    for(_i, line) in lines.enumerate() {
        
        
        let parsed_temp = get_temp(line);
        if parsed_temp > biggest_temp {
            biggest_temp = parsed_temp;
        }
        if parsed_temp < smallest_temp {
            smallest_temp = parsed_temp;
        }
        sum += parsed_temp;
        count += 1;
    }

    println!("\nThe highest temperature was {biggest_temp}");
    println!("The lowest temperature was {smallest_temp}");
    let mean = sum / count as f32;
    println!("The mean temperature was {mean}")
}

fn get_temp(line: &str) -> f32 {
    println!("Linha que chegou: {line}");
    
    let (_, temp) = line.split_once(';').unwrap_or_else(|| {
        eprintln!("An error occured while reading a line from the file");
        println!("Linha: {line}");
        std::process::exit(2);
    });
    
    match temp.trim().parse::<f32>() {
        Ok(temp) => temp,
        Err(err)   => {
            eprintln!("Erro: {err}");

            println!("Temperatura: {temp}");
            println!("Linha: {line}");

            std::process::exit(1);
        }
    }
}