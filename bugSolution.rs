fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Solution 1: Clone the vector before modification
    let vec_clone = vec.clone();
    vec_clone.push(4);

    let mut iter = vec.iter();
    println!("First element: {}", iter.next().unwrap());
    println!("Second element: {}", iter.next().unwrap());
    println!("Vector after cloning: {:?}", vec_clone);

    //Solution 2: Create a new vector for modifications
    let mut vec2 = vec.clone();
    let mut new_vec = Vec::new();
    for x in vec2{
        new_vec.push(x*2)
    }
    println!("New Vector: {:?}",new_vec);
}