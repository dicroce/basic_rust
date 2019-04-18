
pub fn run() {
    let mut count = 0;

    // inifinite loop
    print!("Number: ");
    loop {
        count += 1;
        print!("{} ", count);

        if count == 20 {
            break;
        }
    }
    println!("");

    // while loop fizz buzz
    while count <= 100 {
        if count % 15 == 0 {
            print!("fizzbuzz ");
        } else if count % 3 == 0 {
            print!("fizz ");
        } else if count % 5 == 0 {
            print!("buzz ");
        } else {
            print!("{} ", count);
        }

        count += 1;
    }
    println!("");

    // for loop fizz buzz
    for x in 0..100 {
        if x % 15 == 0 {
            print!("fizzbuzz ");
        } else if x % 3 == 0 {
            print!("fizz ");
        } else if x % 5 == 0 {
            print!("buzz ");
        } else {
            print!("{} ", x);
        }
    }
    println!("");
}
