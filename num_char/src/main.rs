use std::io;
fn check_num(num: i32) -> String {
    if num == 0 {
        return "Zero".to_string();
    }

    if num > 0 {
        if num % 2 == 0 {
            return "Positive even".to_string();
        } else {
            return "Positive odd".to_string();
        }
    } else {
        if num % 2 == 0 {
            return "Negative even".to_string();
        } else {
            return "Negative odd".to_string();
        }
    }
}
fn main(){
    println!("Enter the num");

    let mut num = String::new();
        io::stdin()
        .read_line(&mut num)
        .expect("Failed to expect the line");

    let num:i32 = num.trim().parse().expect("Failed to read");

    let validate = check_num(num);
    println!("{:?}",validate)

}