use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn random_number(name: String) {
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
