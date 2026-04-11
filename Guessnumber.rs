use std::io;
fn main(){
    println!("Guess ther Number");

    println!("Enter the number");


    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");


    println!("Your guessed number is {guess}");



}
