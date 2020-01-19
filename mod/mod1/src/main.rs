pub mod printing {
   pub mod counter {
           pub fn youre_age (age:u32){
                let current_year = 2020 ;
                println!("your age {}",current_year - age);
    }
   }
}
use std::io;

fn main() {
    println!("Please Enter your birth year");
    
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age :u32 = age.trim().parse().expect("Please enter Integer");
        printing::counter::youre_age(age);

}
