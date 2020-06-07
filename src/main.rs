use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hey all!");

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

