fn main() {
    //new String!
    let mut string1 = "Hello, ".to_string();
    //let mut string1 = String::from("Hello, ");

    //"Hello, " is an String Slice: &str.
    //&string1 would be a &String, and the compiler translates it into a &str.
    //either them are on the heap, &str is immutable, String are not, but either them you can pass the ownership

    //string2 gets an reference, a slice, it is immutable.
    let string2 = "Hello, world!";

    //this would move the string1 to the function
    //change1(string1);
    change1(&mut string1);

    //&str are immutable, because it is a reference!
    //change2(string2);

    println!("{} || {}", string1, string2);
}

fn change1(s: &mut String) {
    s.push_str("world!");
    //s.push('w'); this can push a char to the string!

    //other possible way would be return:
    //s + ("world!".to_string())
    //or either return:
    //format!("{}{}", s, "world!")
}
