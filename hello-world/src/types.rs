pub fn basics() {
    // Shadowing
    let _a = "arjun";
    let _a: i32 = 5;

    // Integer and float types
    let _a: i8 = 10;
    let _a: f64 = 23.12;

    // Character types
    let _a: char = 'a';

    // Tuples
    let _a: (i32, i64, char) = (50, 100, 'a');
    let (_a1, _a2, _a3) = _a;
    let _first = _a.0;
    // Arrays
    let _all = ["arj", "arju", "arjun"];
    // Array of fixed length of 1. Stored on the stack, not the heap.
    let _a: [i16; 1] = [5];

    // Need to define types when many are possible
    let parsed: u32 = "43 ".trim().parse().expect("Could not parse!");
    println!("{}", parsed);

    // Expressions & functions
    let _y = {
        let x = 5;
        add_one(x)
    };
}

pub fn slices() {
    let a = String::from("Hello World");
    let len = a.len();
    // These are immutable references to parts of a;
    let b = &a[..3];
    let c = &a[2..len];
    println!("{} {}", b.len(), c.len());

    // Cannot create a mutable reference to a now:
    // let mut d = &mut a;
}

fn add_one(val: i32) -> i32 {
    val + 1
}

