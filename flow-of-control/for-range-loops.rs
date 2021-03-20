// for in construct can be used to iterate through an Iterator
// easiest way to create an Iterator is through range notation i.e. 1..101 = 1 through 100
fn main() {
    println!("\n\nFOR AND RANGE:\n");
    // n will take the values: 1, 2, ..., 100 in each iteration
    // to make it inclusive of the last value the range notation is 1..=100
    // example (will yield the same result): for n in 1..=100 {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    println!("\n\nFOR AND ITERATORS:\n");

    // into_iter, iter and inter_mut all handle conversion of a collection into an Iterator

    // iter borrows each element of the collection through each iteration
    // thus leaving the collection untouched and available for reuse after the loop
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    // into_iter consumes the collection so that on each iteration the exact data is provided
    // after the collection has been consumed it is no longer available for reuse as it has been moved
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // uncommenting this will result in an error as the names variable has moved
    // println!("names: {:?}", names);

    // iter_mut mutably borrows each element of the collection allowing for the collection to be modified in place

    let mut mut_names = vec!["Bob", "Frank", "Ferris"];

    for name in mut_names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", mut_names);
}
