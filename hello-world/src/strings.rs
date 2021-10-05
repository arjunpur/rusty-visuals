pub fn basics() {
    // These are string literals. They are of fixed size, are stored on the stack
    let _a = "hello";
    // These are more complex data types that are stored on the heap
    let _a: String = String::from("hello");
    let mut _a = "world".to_string();
    _a.push_str("hello");

    let _b = "arjun".to_string();
    let _c = "puri";
    let _d = _b + _c;

    let final_str = _d;
    println!("{}", final_str);
}
