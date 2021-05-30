pub fn run() {
    let mut count = 0;

    // Infinity loop
    loop {
        count += 1;
        println!("{}", count);

        if count == 20 {
            break;
        }
    }

    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else {
            println!("{}", count);
        }

        count += 1;
    }
}
