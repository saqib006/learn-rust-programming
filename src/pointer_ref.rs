// refrence pointer - point to a resource in memory

pub fn run(){
    // primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    // with non-premitives, if you assign another variable to a piece of data, the first
    // variable will no longer hold that value, you'll need to use a reference (&) to point to the resource


    // vector

    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));
}