//T is a generic type; it can be anything,
//anything that has the traits PartialOrd and Copy from std.
//list is a reference to an array of T's.
//it is a reference, because if it wasn't, the compiler would not know the size of the array at compile time.
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = &item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![42, 12, 0, 100, 55];
    println!("numbers: {}", largest(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("chars: {}", largest(&char_list));
}

//You can use "impl {Trait name}" as a type, and use "()" to isolate it.
//so you can use generic types (that implement some trait/properties/methods).

//This declares a trait, and add a method to any "class" that "impl Summary" as type
//pub trait Summary {
//    fn summarize(&self) -> String;
//}
