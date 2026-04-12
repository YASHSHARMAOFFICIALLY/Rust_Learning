use std::io;
pub fn sum_even(a:i32,b:i32)->i32{

    let mut sum = 0;
    for  i in a..=b {
        if i % 2 == 0 {
            sum  = sum + i;
        }
    }
   sum 

}

fn main(){
    println!("Enter the number ");

    println!("Enter the first number");

    let mut numa = String::new();
            io::stdin()
            .read_line(&mut numa)
            .expect("Failed to read line");

    let numa = numa.trim().parse().expect("Failed to read input");

    let mut  numb = String::new();
        io::stdin()
            .read_line(&mut numb)
            .expect("Failed to read line");
    
        let numb = numb.trim().parse().expect("Failed to read input");

    let sum = sum_even(numa,numb);

    println!("The value is {sum}");
}
