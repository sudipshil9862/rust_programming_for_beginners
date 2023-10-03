fn main() {
    let str1 = "Hello ";
    let str2 = "world!";
    let concatenated_string = str1.to_string() + str2; //string concatenation requires an owned `String` on the left
    println!("{}", concatenated_string);
}
