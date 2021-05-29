pub fn run() {
    let mut hello = String::from("Hello ");

    println!("x Capacity: {}", hello.capacity());

    println!("{}", hello);
    println!("Length: {}", hello.len());

    hello.push('W');
    hello.push_str("orld!");

    println!("{}", hello);
    println!("Length: {}", hello.len());

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // check if empty
    println!("Is empty: {}", hello.is_empty());

    // contains
    println!("Contains: {}", hello.contains("World!"));

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}
