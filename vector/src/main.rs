// 1. Space-separated input (most common)

use std::io;
// fn main(){
//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .unwrap();

//     let vec:Vec<i32> = input
//         .trim()
//         .split_whitespace()
//         .map(|x| x.parse().unwrap())
//         .collect();

//     println!("{:?}",vec)
// }


//2. Fixed size vector (first input = size)
fn main(){
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n:usize = n.trim().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let vec:Vec<i32> = input 
        .trim()
        .split_whitespace()
        .take(n)
        .map(|x| x.parse().unwrap())
        .collect();

        println!("{:?}",vec);
}


//3 Taking input line-by-line into vector
use std::io;

fn main(){
    let mut n = String::new();
        io::stdin().read_line(&mut n).unwrap();
    let n :usize = n.trim().parse().unwrap();

    let mut vec = Vec::new();

    for _ in 0..n {
        let mut input = String::new()
        io::stdin().read_line(&mut input).unwrap();
        let num: i32 = input.trim().parse().unwrap();
        vec.push(num);
    }
    println!("{:?}",vec);
}

