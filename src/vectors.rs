// vectors resizable arrays

use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec! [1, 2, 3, 4, 5];

    // reassign value

    numbers[2] = 20;

    // add on to vector
    numbers.push(6);
    numbers.push(7);

    numbers.pop();

    println!("{:?}", numbers);

    // get single value
    println!("single value {}", numbers[0]);

     // get vector length
     println!("vector length {}", numbers.len());

     // vector are stack allocated
     println!("vector  occupies {} bytes", mem::size_of_val(&numbers));

     // get slice

     //let slice: &[i32] = &numbers;
     let slice: &[i32] = &numbers[0..3];

     println!("Slice :{:?}", slice);

    // loop through vector values

     for x in  numbers.iter() {
         println!("Numbers: {}", x)
     }

     // loop and mutate values

     for x in numbers.iter_mut() {
         *x *= 2;
     }
     println!("Numbers Vec: {:?}", numbers);


}