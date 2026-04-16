use std::io;

fn main(){
    println!("Enter the  number ");


    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let vec:Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

println!("{:?}",vec);
}