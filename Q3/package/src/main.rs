use mylib;
use std::io;

fn main() {
    println!("let's count your name length");
    println!("Please Enter your name");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    mylib::printing::counter::length(input);
   }