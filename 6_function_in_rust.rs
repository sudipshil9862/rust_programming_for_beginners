fn main() {
    print_labeled_measurement(206, 'F');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The unicode is: {value}{unit_label}");
}
