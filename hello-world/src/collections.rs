pub fn basics() {
    let mut col: Vec<u32> = Vec::new();
    col.push(32);
    println!("length: {}", col.len());

    let mut c = vec![1, 2, 3, 4];
    let third: &i32 = &c[2];
    println!("third element is: {}", third);

    let value = &c[0];

    // Uncommenting this won't compile due to Rust's
    // Borrowing rules.
    // c.push(5);

    println!("try to access value: {}", value);

    // Iterating through a vector
    for v in &c {
        println!("printing values: {}", v);
    }

    // We can make changes too via mutable references
    for v in &mut c {
        *v += 50;
    }
}

#[derive(Debug)]
enum Container {
    IntValue(i32),
    StringValue(String),
}

pub fn enum_vectors() {
    let v = vec![
        Container::IntValue(5),
        Container::StringValue(String::from("arjun")),
    ];
    for c in &v {
        println!("container value in vector: {:?}", c);
    }
}
