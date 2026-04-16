use std::io;


pub fn convert_temperature(value:f64,from_unit:&str,to_unit:&str)->Result<f64,String>{
   match (from_unit.to_uppercase().as_str(), to_unit.to_uppercase().as_str()) {
        ("C", "F") => Ok((value * 9.0 / 5.0) + 32.0),
        ("C", "K") => Ok(value + 273.15),
        ("F", "C") => Ok((value - 32.0) * 5.0 / 9.0),
        ("K", "C") => Ok(value - 273.15),
        (a, b) if a == b => Ok(value), // Same unit
        _ => Err(format!("Unsupported conversion: {} to {}", from_unit, to_unit)),
    }
}


fn main(){
    println!("Welcome to temperature converter");


    println!("Enter the temperature value");

    let mut  temp = String::new();
        io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read value");

    let temp:f64 = temp.trim().parse().expect("Failed to read value");

    println!("Enter the Current unit of temeprature");

    let unit1 = String::new();
        io::stdin()
            .read_line(&mut unit1)
            .expect("Failed to Read the unit");
    
    let unit1 = unit1.trim().expect("Failed to do this");

    println!("Enter the temeprature you want to convert");
    let unit2 = String::new();
        io::stdin()
            .read_line(&mut unit2)
            .expect("Failed to read line");
    
    let unit2 = unit2.trim().expect("Failed to do this");

    let reult = convert_temperature(temp,unit1,unit2);
    println!("{:?}",reult);
        

}