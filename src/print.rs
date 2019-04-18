
pub fn run() {
    // basic printing to console (note exclamation point in name indicates its a macro).
    println!("Hello from print.rs file.");

    // basic formatting
    println!("{} is from {}.", "Alice", "Nevada");

    // positional arguments
    println!("{0} is from {1} and {0} likes to {2}.", "Alice", "Nevada", "code");

    // named arguments (not allowed on functions generally, but possible with macros)
    println!("{name} likes to play {activity}.", name = "Alice", activity = "Red Dead Redemption 2");

    // debug trait
    // traits are like a package of functions a type must implement if it implements that trait. The "debug"
    // trait returns a string representation of the type suitable for printing in the context of debugging.
    // We haven't learned about tuples yet but the inner parenthesis below create a tuple with 3 elements (i32,
    // bool, and &str). The {:?} tells println! to call the debug trait on the passed object to get the string
    // to inject. Builtin tuples implement the debug trait so this all works out.
    println!("{:?}",(12, true, "hello"));

    // math
    println!("10 + 10 = {}", 10 + 10);
}