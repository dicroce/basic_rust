
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // re-assign value
    numbers[2] = 20;

    // append to vector
    numbers.push(5);
    numbers.push(6);

    // pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // get a single value
    println!("Single Value: {}", numbers[0]);

    // get vector length
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // basic iteration
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // iterate and mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}
