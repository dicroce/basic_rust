
pub fn run() {

    // Tuples can be created simply by wrapping some literals and/or variables in parenthesis.
    let person = ("Tony", "California", 42);

    // Tupe field are accessed by index with . syntax
    println!("{} is from {} and is {}.", person.0, person.1, person.2);
}
