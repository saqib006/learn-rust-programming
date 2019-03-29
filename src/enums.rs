// enums are types which have a few definite values

enum Movement{

    // variants

    Up,
    Down,
    Left,
    Right
}

fn move_avator(m:Movement){
    //perform action depending on info
    match m {
        Movement::Up => println!("Avator moving up"),
        Movement::Down => println!("Avator moving down"),
        Movement::Left => println!("Avator moving left"),
        Movement::Right => println!("Avator moving right")
    }
}


pub fn run(){
    let avator1 = Movement::Left;
    let avator2 = Movement::Right;
    let avator3 = Movement::Up;
    let avator4 = Movement::Down;

    move_avator(avator1);
    move_avator(avator2);
    move_avator(avator3);
    move_avator(avator4);
}