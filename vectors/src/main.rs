fn main() {
    //If I didn't add 1, 2 and 3 to the vector, I would need to especify ":Vec<i32>" to the compiler know the type of the vector.
    let mut v = Vec::new();
    //let mut v = vec!([1, 2, 3]);
    v.push(1);
    v.push(2);
    v.push(3);

    //I am using & to make them references, so I don't pass the ownership to println!.
    //let third: &i32 = &v[2];
    let third = v.get(2).unwrap();

    println!("The third element on v is: {third}");

    //If I don't reference (&) and then dereference (*), a function call inside the "for" implementation would get the ownership of v.
    for item in &mut v {
        *item += 2
    }

    let third = v.get(2).unwrap();
    println!("The third element on v is: {third}");
}
