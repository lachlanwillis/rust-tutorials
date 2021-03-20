// Some use cases, when matching enums, match is awkward

// Make `optional` of type `Option<i32>`
// let optional = Some(7);

// match optional {
//     Some(i) => {
//         println!("This is a really long string and `{:?}`", i);
//         // ^ Needed 2 indentations just so we could destructure
//         // `i` from the option.
//     },
//     _ => {},
//     // ^ Required because `match` is exhaustive. Doesn't it seem
//     // like wasted space?
// };

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

// if let is cleaner for this use case and allows various failure options to be specified
fn main() {
    // All have type Option<i32>
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specifiy a failure use an else
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        // Destructure failed, change to the failure case
        println!("Didn't match a number, let's go with a letter!");
    }

    // Provide an altered failing condition
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an `else if` condition to see if the
    // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number, lets go with a letter!");
    } else {
        // The condition evaluated false, this is the default branch
        println!("I don't like letters lets go with an emoticon! :)");
    }

    // if let can also be used to match any enum value
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is a foobar");
    }

    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // Variable c matches Foo::Qux which has a value similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with `if let`
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
}
