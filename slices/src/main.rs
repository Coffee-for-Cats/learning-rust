fn main() {
    //"ô" uses 2 bytes.
    let s = String::from("hellô world!");

    //so I need to add one more byte here, wtf.
    let fw = first_word(&s);

    println!("{fw}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
