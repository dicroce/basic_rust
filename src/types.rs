
pub fn run() {

    // default integer type
    let x = 1;

    // default floating point type
    let y = 2.5;

    // add explicity type
    let z: i64 = 45454545454545;

    // max's
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // boolean
    let is_active = true;

    // boolean from expression
    let is_greater = 10 > 5;

    // utf8 code points
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}