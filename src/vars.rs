// variables hold primitive data or refrences to data
// variables are immutable by default
// rust is a block-scoped language

pub fn run(){
    let name = "Saqib";
    let mut age = 37;
    println!("My name is {} and i am {}", name, age);
    age = 24;
    println!("My name is {} and i am {}", name, age);



    // Define constant
    const ID: i32 = 801;
    println!("ID: {}", ID);

    // assin multiple vars

    let (my_name, my_age) = ("saqib", 24);
    println!("{} is {}", my_name, my_age);
}