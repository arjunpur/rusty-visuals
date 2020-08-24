// Define a module with the `mod` keyword. Modules can
// hold definitions of other items such as structs,
// enums, constants and traits too.
//
// We don't need the `mod` keyword here because all functions are defined
// in this ownership.rs file
pub fn basics() {
    let a = 5;
    let b = a;

    // a is copied onto the stack and assigned to b
    println!("{}", a);
    println!("{}", b);

    let c = String::from("Hello World");
    let d = c;
    // c is no longer valid as d now owns the pointer to the piece of memory
    // println!("{}", c);
    println!("{}", d);
}

pub fn references() {
    let new_c = String::from("Hello World");
    let new_d = new_c.clone();

    println!("{} = {}", new_c, new_d);

    let a = String::from("Arjun");
    // create immutable references to a.
    let a_ref = &a;
    let b_ref = &a;

    println!("{}", append_hello(a_ref));
    println!("{}", append_hello(b_ref));

    // Create a mutable reference to a string, and pass it into
    // a function that mutates it.
    let mut b = String::from("Hello");
    let mut_ref = &mut b;
    update_hello(mut_ref)
}

// Pass in an immutable reference and use it
fn append_hello(a_ref: &String) -> String {
    let mut hello = String::from("Hello ");
    hello.push_str(a_ref);
    hello
}

fn update_hello(mut_str: &mut String) {
    mut_str.push_str(" world");
}

