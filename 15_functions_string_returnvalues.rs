fn main() {
    let s1 = String::from("hellos1");
    takes_ownership(s1);
    //println!("s1:{s1}"); //s's value moves into the function so s is no longer valid
    
    let s2 = String::from("hellos3");
    let s3 = takes_and_gives_ownership(s2);
    println!("s3: {s3}");
    
    let s4 = String::from("hellos5");
    let (s5,len) = calculate_s5_and_len(s4);
    println!("The length of '{}' is {}.", s5, len);

    let x = 5;
    makes_copy(x);
    println!("x: {x}");
}

fn takes_ownership(some_string: String) {
    println!("s1: {}", some_string);
}

fn takes_and_gives_ownership(some_string: String) -> String {
    some_string
}

fn makes_copy(some_integer: i32) {
    println!("x: {}", some_integer);
}

fn calculate_s5_and_len(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}
