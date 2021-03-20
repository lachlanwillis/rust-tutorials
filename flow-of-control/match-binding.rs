fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I still haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1..=12 => println!("I'm a child of the age: {}", n),
        n @ 13..=19 => println!("I'm a teen of the age: {}", n),
        // nothing bound, return the result
        n => println!("I'm an old person of the age {}", n),
    }

    // You can also use binding to destructure enum variants, such as Option
    match some_number() {
        // Got `Some` variant, match if its value, bound to `n`, is equal to 42.
        Some(n @ 42) => println!("The answer: {}!", n),
        // Match any other number
        Some(n) => println!("Not interesting... {}", n),
        // Match anything else (None variant)
        _ => (),
    }
}
