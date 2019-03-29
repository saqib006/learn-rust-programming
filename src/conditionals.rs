// conditionals - used to check the condition of something and act onthe result

pub fn run(){
    let age:u8 = 30;
    let check_id: bool = true;
    let knows_person_age = true;

    // if/else

    if age >= 21 && check_id || knows_person_age{
        println!("age is greater than 22")
    }else if age < 21 && check_id{
        println!("age is not greater than 22")
    }else{
        println!("else")
    }

    // shorthand if
    let is_of_age = if age >= 21 {true} else {false} ;
    println!("age: {}", is_of_age);
}