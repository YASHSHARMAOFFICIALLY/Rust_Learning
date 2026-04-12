use std::io;

pub fn countdown(n:u32)->Vec<u32>{
    let mut result = Vec::new();

    for i in (0..=n).rev(){
        result.push(i);
    }
    result
}



fn main(){
    println!("Enter the number");

    let mut input = String::new();
       io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim().parse().expect("failed to read input");


    let countdownn = countdown(input) ;
    println!("{:?}",countdownn);
}