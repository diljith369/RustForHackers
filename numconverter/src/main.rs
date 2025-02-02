use std::io;

fn main() {
    println!("Enter a number to convert:");
    let mut input = String::new();
    let read_input = io::stdin();
    read_input.read_line(&mut input).expect("Failed to read line");
    let number: i32 = input.trim().parse().expect("Invalid input");
    println!("Binary of {} is: {:b}", number, number);
    println!("Hexadecimal of {} is: {:x}", number,number);
    println!("Octal of {} is: {:o}", number,number);
}


