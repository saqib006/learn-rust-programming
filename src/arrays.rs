// arrays fixed list where elements are the same data types

use std::mem;

pub fn run(){
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // reassign value

    numbers[2] = 20;

    println!("{:?}", numbers);

    // get single value
    println!("single value {}", numbers[0]);

     // get array length
     println!("array length {}", numbers.len());

     // arrays are stack allocated
     println!("array  occupies {} bytes", mem::size_of_val(&numbers));

     // get slice

     //let slice: &[i32] = &numbers;
     let slice: &[i32] = &numbers[0..3];

     println!("Slice :{:?}", slice);


}