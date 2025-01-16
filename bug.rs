fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();

    println!("First element: {}", iter.next().unwrap()); // Prints 1

    // Modifying the vector while iterating over it is undefined behavior
    vec.push(4);

    println!("Second element: {}", iter.next().unwrap()); // This might panic or give unexpected results
}