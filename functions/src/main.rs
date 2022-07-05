fn main() {
    let x = another_function();
    println!("x is: {x}");

    //This is a block; you can execute code within another context, and "return" something.
    //It's like a imediately called function from javascript, but with a hidden return, where we don't have the semicolon
    let y = { 4 + 1 };
    println!("y is: {y} too!")
}

fn another_function() -> i32 {
    4 + 1 //; //you cannnot have a ";", because if so, you will return nothing!
          //^ its the same as:
          //return 4 + 1
}
