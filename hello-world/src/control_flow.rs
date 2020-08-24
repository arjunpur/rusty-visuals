pub fn basics() {
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

