// Rust has a few reserved lifetime names. One of those is 'static. You might encounter it in two situations:
// A reference with 'static lifetime:
// let s: &'static str = "hello world";

// 'static as part of a trait bound:
// fn generic<T>(x: T) where T: 'static {}

// Both are related but subtly different and this is a common source for confusion when learning Rust.
// Here are some examples for each situation:

// REFERENCE LIFETIME:
// As a reference lifetime `'static` indicates that the data pointed to by the reference lives for
// the entire lifetime of the running program. It can still be coerced to a shorter lifetime.

// There are two ways to make a variable with 'static lifetime, and both are stored in the read-only memory of the binary:
// Make a constant with the static declaration.
// Make a string literal which has type: &'static str.

// Make a constant with `'static` lifetime.
static NUM: i32 = 18;

// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Make a `string` literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}
