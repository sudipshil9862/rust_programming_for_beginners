fn main(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("length of {} is: {}", s1, len);

    let mut s2 = String::from("mutable string");
    change(&mut s2);
    //Mutable references have one big restriction
    //if you have a mutable reference to a value, you can have no other references to that value.
    println!("changed s2: { }", s2);
}

fn calculate_length(s: &String) -> usize{
    s.len() //non-mutable string
}

fn change(s: &mut String){
    s.push_str(" changed");
}
