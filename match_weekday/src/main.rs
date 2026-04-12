use std::io;


pub fn weekday_from_number(day:u8) {
    match day {
        1 =>println!("Monday"),
        2=>println!("Tuesday"),
        3=>println!("Wednesday"),
        4=>println!("Thirsday"),
        5=>println!("Friday"),
        6=>println!("Saturday"),
        7=>println!("Sunday"),
         _ => println!("Invalid day"),
    }
}

fn main(){
    println!("Enter the number");

    let mut num = String::new();
        io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num = num.trim().parse().expect("failed to read input");

    let weekday = weekday_from_number(num);

    println!("{:?}",weekday)
}