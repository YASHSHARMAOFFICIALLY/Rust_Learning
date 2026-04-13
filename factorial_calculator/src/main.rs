use std::io;



fn factorial_number(num:i32)->i32{
    let mut fact = 1;
    for i in 1..=num {
        
            fact = fact * i;
       
    };
    fact 
}

fn main(){
    println!("Enter the number you want to calculate for factorial");

    let mut num = String::new();
        io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num = num.trim().parse().expect("Failed to read line");

    let factorial = factorial_number(num);
    println!("{factorial}")
}