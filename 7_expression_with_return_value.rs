fn main() {
    let y = {
        let x = 3;
        x + 2
    };
    //NOTE: x+2 doesnot have any semicolon
    println!("expression return value: {y}");
}
