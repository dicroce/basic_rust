
use std::mem;

pub fn run() {

    let mut numbers: [i32; 4] = [1, 2, 3, 4];
    //let mut numbers = [1, 2, 3, 4];                     without type and size hint

    // re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // get a single value
    println!("Single Value: {}", numbers[0]);

    // get array length
    println!("Array length: {}", numbers.len());

    // arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];           
    //let slice = &numbers[0..2];                         without type hints
    println!("Slice: {:?}", slice);
}
