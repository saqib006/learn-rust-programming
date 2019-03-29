// structs - used to create custom data types


// tradional struct

/*struct Color{
        red:u8,
        green:u8,
        blue:u8
    }

// tuple struct
struct Color2(u8,u8, u8);*/

struct Person{
    first_name:String,
    last_name:String
}

impl Person {
    // construct person
    fn new(first:&str, last:&str) -> Person {
        Person{
            first_name:first.to_string(),
            last_name:last.to_string()
        }
    }

    // get full name
    fn full_name(&self)-> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn set_last_name(&mut self, last:&str){
    self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }
}

pub fn run(){
  /*  let mut c = Color{
        red:255,
        green:0,
        blue:0
    };
    c.blue = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c2 = Color2(255,0,0);
    println!("Color: {} {} {}", c2.0, c2.1, c2.2);*/

    let mut p = Person::new("Saqib", "Ahmed");
    println!("Person {}",p.full_name() );
    p.set_last_name("Khan");
    println!("Person {}", p.full_name() );
    println!("Person Tuple {:?}", p.to_tuple() );
}