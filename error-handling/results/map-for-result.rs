// Panicking in the previous example's multiply does not make for robust code
// Generally, we want to return the error to the caller so it can decide what is the right way to respond to errors

// We first need to know what kind of error type we are dealing with
// To determine the Err type, we look to parse(), which is implemented with the FromStr trait for i32
// As a result, the Err type is specified as ParseIntError

// Example of making the Result check less cumbersome by using the available .map and .and_then combinators
use std::num::ParseIntError;

// As with `Option`, we can use combinators such as `map()`.
// This function reads:
// Modify n if the value is valid, otherwise pass on the error.
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // This still presents a reasonable answer.
    let twenty = multiply("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = multiply("t", "2");
    print(tt);
}
