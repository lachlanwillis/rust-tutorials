// Macros look like functions except their name ends with a bang '!', but instead of generating a function call,
// they are expanded into source code that gets compiled with the rest of the program
// Macros are created using the macro_rules! macro.
// Why are macros useful?
// - Don't repeat yourself
// - Domain-specific languages
// - Variadic interfaces

// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}

fn main() {
    // This call will expand into `println!("Hello");`
    say_hello!()
}
