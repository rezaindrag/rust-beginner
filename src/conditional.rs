pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;

    if age >= 21 && check_id {
        println!("Bartender: what do you want to drink?")
    } else if age < 21 && check_id {
        println!("Bartender: you have to leave")
    } else {
        println!("Bartender: I need to see your id")
    }

    // Shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("{}", is_of_age);
}
