use std::io;
fn main(){
    println!("Welcome to Temperature converter ");

    println!("Enter the Temperature Value ");

    let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read value ");

    let temp:f64 = temp.trim().parse().expect("please enter a value");
    
    println!("the temperature value is {temp}");

    println!("Enter the degree you want to convert the temperature (C/F) ");

    let mut degree = String::new();
        io::stdin()
            .read_line(&mut degree)
            .expect("Failed to read value");
    let degree = degree.trim();

    println!("You wanted to convert the temperature in {degree}");

    if degree == "C"{
        let result = (temp-32.0)*5.0/9.0;
            println!("Converted temperature is {} C",result);
        
    }else if degree == "F"{
        let result = (temp*9.0/5.0) + 32.0;
        println!("Converted temperature is {}",result);
    }else {
        println!("Inavlid input");
    }







}