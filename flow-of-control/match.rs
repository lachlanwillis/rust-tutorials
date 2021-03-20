// rust provides pattern matching via the match keyword which is used similar to switch in other languages
// the first matching arm is evaluated and all possibilities must be covered

fn main() {
    let number = 13;

    println!("Tell me about {}", number);

    match number {
        // match a single value
        1 => println!("one"),
        // match several values
        2 | 3 | 5 | 7 | 11 => println!("this is a prime"),
        // match an inclusive range
        13..=19 => println!("a teen"),
        // handle the rest of cases
        _ => println!("aint special"),
    }

    let boolean = true;
    // match is an expression too!
    let binary = match boolean {
        // the arms of a match must cover all possible values
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary)
}
