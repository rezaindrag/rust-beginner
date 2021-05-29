pub fn run() {
    // print to console
    println!("hello");

    // basic formatting
    println!("{} is from {}", 1, 2);

    // positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // named arguments
    println!("{name} likes to play {activity}", name="John", activity="Baseball");

    // placeholder traits
    println!("Binary: {:b} Hex {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("{}", 10 + 10);
}
