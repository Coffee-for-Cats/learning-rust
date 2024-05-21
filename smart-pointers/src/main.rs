// Enums always have the maximum size possible in memory.
// Recursive = possible inifite space! But...
// boxes are fixed in size, because it's a pointer.
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // the box is literally a pointer.
    // controlled by another guy's code.
    // let b = Box::new(5);
    // println!("b = {}", b);

    let list = Cons(1, 
        Box::new(Cons (2, 
            Box::new(Cons (3,
                Box::new(Nil
            ))
        ))
    ));
}
