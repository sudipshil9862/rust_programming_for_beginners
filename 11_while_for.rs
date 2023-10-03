fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("using while: the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("using for: the value is: {element}");
    }
}
