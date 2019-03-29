// functions - used to store blocks of code for re-use

pub fn run(){
    greeting("Hello", "Rust");


    // bind functions values to variables
   let get_sum = add(5, 5);
   println!("Sum {}",  get_sum);

   // closure
    let n3:i32 = 10;
   let add_nums = |n1:i32, n2:i32 | n1 + n2 + n3;
   println!("C Sum: {}", add_nums(3, 3) );
}

fn greeting(greet:&str, name:&str){
    println!("{} {}, nice very nice", greet, name );
}


// return function
fn add(n1: i32, n2:i32) -> i32 {
    n1 + n2
}