
pub fn run() {

    // variables are immutable by default
    let name = "Tony";

    // variables marked mut are mutable
    let mut age = 41;

    println!("My name is {} and I am {}", name, age);

    age = 42;

    println!("My name is {} and I am {}", name, age);

    // define constant - constants require type
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // multiple variable assigment
    let (my_name, my_age) = ("Tony", 42);
    println!("{} is {}.", my_name, my_age);

}