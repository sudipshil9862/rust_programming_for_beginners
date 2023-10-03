//user input: index number
//output: array[index] value
//if wrong index then it'll through error

//When you attempt to access an element using indexing, Rust will check that the index you’ve specified is less than the array length
//This is an example of Rust’s memory safety principles in action

use std::io;
fn main(){
    let a = [1,2,3,4,5];
    println!("please enter an array index: ");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read lin");
    //check user input index is a number or not
    let index: usize = index
        .trim()
        .parse()
        .expect("index entered was not a number");
    let element  = a[index];
    println!("the value of the element at index {index} is: {element}");
}
