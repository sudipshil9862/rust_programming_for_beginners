fn func_new(x: i32) -> i32 {
    x + 5
    //no semicolon to return value
}

fn main(){
    let x = func_new(5);
    println!("value found: {x}");
}
