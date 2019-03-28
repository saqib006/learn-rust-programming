// primitive str = immutable fixed-length smewhere in memory

// string = growable, heap-allocated data structure - use when you need to modify or own string data

pub fn run(){
    let _hello1 = "hello "; // immutable fixed-length
    let mut hello2 = String::from("Hello ");

    // get length

    println!("{}", hello2.len());
    hello2.push('W');

    hello2.push_str("orld");

    // capacity in bytes
    println!("Capacity: {}", hello2.capacity());

    println!("is Empty: {}", hello2.is_empty());

    // contains
    println!("Conatin 'World': {}", hello2.contains("World"));

    // replace
    println!("Conatin 'World': {}", hello2.replace("World", "Rust"));

    // loop through string by whitespace
    for word in hello2.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    println!("{}", hello2);

    assert_eq!(2, s.len()); // only show if there is errors
    assert_eq!(10, s.capacity());

    println!("{}", s);
}