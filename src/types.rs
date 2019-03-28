/*
Primitive Types
Integers: u8, i8, u16, u32, i32, u64, i64, u128, i128 ( number of bites take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

pub fn run(){
    //Defaults is i32
    let x = 1;

    // default is f64
    let y = 2.5;

    // add explicit type
    let z:i64 = 454554454545;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // boolean
    let is_active: bool = true;

    // get boolean from expression
    let is_greater: bool = 10 > 5;

    let a1:char = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}