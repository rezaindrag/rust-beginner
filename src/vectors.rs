// Vector is resizable array

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    println!("{:?}", numbers);

    println!("{}", numbers[0]);

    numbers[2] = 20;

    numbers.push(7);
    numbers.push(8);

    println!("{}", numbers[2]);

    // Vector are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x)
    }

    // loop & mutate values
    for x in numbers.iter_mut() {
        // What is *x?
        *x *= 2;
    }

    println!("Numbers vec: {:?}", numbers)
}
