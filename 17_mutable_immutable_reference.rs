fn main(){
    /*
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
    */
    //We also cannot have a mutable reference while we have an immutable one to the same value

    let mut s = String::from("mutable");

    let r1 = &s;
    let r2 = &s;
    println!("no-{}-r1 and no-{}-r2", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s;
    println!("{}-r3", r3);
}
