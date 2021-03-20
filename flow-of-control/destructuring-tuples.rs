fn main() {
    let triple = (0, -2, 3);

    println!("tell me about {:?}", triple);

    // match used to destructure tuple
    match triple {
        // destructure the second and third elements
        (0, y, z) => println!("first is 0, y is {:?}, z is {:?}", y, z),
        // .. can be used to ignore the rest of the tuple
        (1, ..) => println!("first is 1 and the rest doesnt matter"),
        // _ means dont bind the value to a variable
        _ => println!("it doesnt matter what they are"),
    }
}
