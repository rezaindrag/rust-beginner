pub fn run() {
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    // Use pointer
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    println!("Values: {:?}", (&vec1, vec2));

    // NOTE: Vector must use a reference (&), because it's non-primitive type

    let n1 = 1;
    let mut n2 = n1;
    println!("{:?}", (n1, n2));
}
