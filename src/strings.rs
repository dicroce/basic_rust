
pub fn run() {

    let mut hello = String::from("Hello ");

    // get length
    println!("Length: {}", hello.len());

    // push char, works because hello is mut
    hello.push('W');

    // push string
    hello.push_str("orld!");

    // capacity in bytes
    println!("Capactity: {}", hello.capacity());

    // emptyness
    println!("is empty: {}", hello.is_empty());

    // contains
    println!("Contains 'World' {}", hello.contains("World"));

    // replace
    println!("Replace: {}", hello.replace("World", "There"));

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing
    assert_eq!(2, s.len());

    println!("{}", s);
}