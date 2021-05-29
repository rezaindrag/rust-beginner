// Array is a fixed list where elements are the same data types

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    println!("{}", numbers[0]);

    numbers[2] = 20;

    println!("{}", numbers[2]);

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers))
}
