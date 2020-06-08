use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    learning_types();
    _run_random_number_game("Arjun".to_string());
}

fn learning_types() {
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

    // Strings
    learning_strings();
    
    // Control flow
    learning_control_flow();

    // Need to define types when many are possible
    let parsed: u32 = "43 ".trim().parse().expect("Could not parse!");
    println!("{}", parsed);

    // Expressions & functions
    let _y = {
        let x = 5;
        add_one(x) 
    };

}

fn add_one(val: i32) -> i32 {
    val + 1
}

fn learning_control_flow() {
    // If conditions
    let _a = 5;
    let _b = 6;
    if _a < _b {
        println!("Lower!");
    }
    let _c = if _b > _a { "wohoo" } else { "nohooo" };
    println!("{}", _c.to_string());
    
    // Loop construct
    let mut counter = 0;
    let mut result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result: {}", result);
    
    // While loops
    while result > 0 {
        result -= 1;
    }
    println!("New result: {}", result);

    // For loops
    result = 0;
    let _arr = [5; 5];
    for elm in _arr.iter() {
        result += elm;    
    }
    println!("Accumulated: {}", result);

    for i in (0..4).rev() {
        println!("{}", i);
    }
}

fn learning_strings() {
    let _a = "hello";
    let _a: String = String::from("hello");
    let mut _a = "world".to_string();
    _a.push_str("hello");

    let _b = "arjun".to_string();
    let _c = "puri";
    let _d = _b + &_c;

    let final_str = format!("{}", _d);
    println!("{}", final_str);
}

fn _run_random_number_game(name: String) {
    println!("Hey {}!", name);

    let rnd = rand::thread_rng().gen_range(1, 101);

    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Error on trying to read_line");

    let number: i32 = number.trim().parse().expect("Could not parse input!");

    match number.cmp(&rnd) {
        Ordering::Greater => println!("Higher!"),
        Ordering::Less => println!("Lower!"),
        Ordering::Equal => println!("Equal!"),
    }
    println!("Your Number was: {}, but we generated: {}", number, rnd);
}

