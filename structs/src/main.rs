struct Tuple(i32, i32, i32);

struct Normal {
    name: String,
    age: i32,
}

fn main() {
    let normal_tuple = (0, 0, 0);
    let tuple_struct = Tuple(0, 0, 0);
    let structure = Normal {
        name: String::from("Lucas"),
        age: 17,
    };
    let arrays = [250, 250, 250];

    println!("ANOTATIONS AND STRUCTURES");
    println!(
        "Tuple and Tuple Structs: {}, {}",
        normal_tuple.0, tuple_struct.1
    );
    println!("Normal Structures: {}", structure.name);
    println!("Arrays: {}", arrays[2]);
}
