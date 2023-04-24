use std::collections::HashMap;
use std::io;

fn main() {
    let mut departments = HashMap::new();

    println!("Welcome to the terminal department manager!");
    see_commands();

    loop {
        //gets the command
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Error reading your line! Try again");

        let command = command.trim();

        //the collect I actually do not know how works, but it makes it a vector!
        let words = command.split(" ").collect::<Vec<&str>>();

        //Gets the verb, the command to be executed.
        let verb = match words.get(0) {
            Some(v) => v,
            None => {
                println!("You have written something wrong, try again!");
                see_commands();
                continue;
            }
        };

        match verb.to_lowercase().as_str() {
            "add" => {
                let name = match words.get(1) {
                    Some(n) => n,
                    None => "",
                };
                let department = match words.get(3) {
                    Some(dep) => dep,
                    None => "",
                };

                if name == "" || department == "" {
                    println!("You have written something wrong, try again!");
                    see_commands();
                } else {
                    add(&mut departments, name, department);
                }
            }
            "see" => match words.get(1) {
                Some(department) => see(&mut departments, department),
                None => {
                    println!("You have written something wrong, try again!");
                    see_commands();
                }
            },
            "exit" => {
                break;
            }
            _other => {
                println!("Error! Incorrect command!");
                see_commands();
            }
        }
    }
}

fn add(departments: &mut HashMap<String, Vec<String>>, name: &str, department: &str) {
    let people = departments.entry(department.to_string()).or_insert(vec![]);
    people.push(name.to_string());

    println!("Added {name} to {department}");
}

fn see(departments: &mut HashMap<String, Vec<String>>, department: &str) {
    let people = departments
        .entry(department.to_string())
        .or_insert(Vec::new());

    println!("Who are on {department} are: ");
    for person in people {
        println!("- {person}");
    }
}

fn see_commands() {
    println!("-> add (name) to (department) | you will add someone to the department");
    println!("-> see (department) | you will see who are on the department");
    println!("-> exit | you will exit the program");
}
