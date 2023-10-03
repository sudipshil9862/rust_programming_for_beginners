fn main() {
    let x = 5;
    let x = x + 1;//created new variable named x and put value x+1 - here x = 6
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // x=12
    }
    println!("The value of x is: {x}"); //x=6
}

