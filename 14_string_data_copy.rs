fn main(){
    let s1 = String::from("hello");
    //let s2 = s1; //s1 contains: ptr,len,capacity and ptr points to index,data
    //when let s2=s1 -> now ptr of s2 points to same index,data of s1 and s1 gets free from memory
    //and s1 cannot be recover
    //so s1 cannot be printed but s2 can be printed
    //but if we still want s1 and need to copy only data to s2 then use .clone()
    let s2 = s1.clone(); //copy only data from s1

    println!("s1: {}, world!", s1);
    println!("s2: {}, world!", s2);
}
