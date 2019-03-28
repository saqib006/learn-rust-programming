pub fn run(){
    // print to console
    println!("Hello World from print file");

    // basic formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Saqib", "Karachi");

    //postional arguments
    println!("{0} is from {2} and love to {1}", "Saqib", "Code", "Karachi");

    //named arguments
    println!("{name} is like to play {activity}",  name="sarfaraz", activity="Cricket");

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    // basic math

    println!("10 + 10 = {}", 10 + 10);
      
}