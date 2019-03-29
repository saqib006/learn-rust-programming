use std::env;
pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Saqib";
    let status = "70%";
   // println!("Command: {}", command);

   if command == "hello"{
       println!("Hi {}, how you doing", name);
   } else if command == "status" {
       println!("Status is {}", status);
   }else {
       println!("not valid command");
   }
}