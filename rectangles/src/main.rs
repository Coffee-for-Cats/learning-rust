//it's needed for printing the struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&mut self) -> u32 {
        self.width += 10;
        self.width * self.height
    }
}

fn main() {
    let mut my_rectangle = Rectangle {
        height: 80,
        width: 30,
    };

    //area() is an implementation of Rectangle; it gets a reference to the object, so I don't have to worry about the ownership
    println!("The area of the rectangle is: {}", my_rectangle.area());

    //debug macro. Can print structs and other things in a not so pretty way
    dbg!(&my_rectangle);
    //or using the pretty operator on println: the {:?}, or {:#?} for breaking lines on the struct
    println!("{:?}", &my_rectangle);
}
