//Vecotr 

//Vector bacic 

// fn main(){
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);

//     println!("{:?}",vec)
// }


//vector plain text even number only 


// fn odd_value(v:&mut Vec<i32>){
//     let mut i = 0;
//     while i < v.len(){
//         if v[i] % 2 == 0 {
//             v.remove(i);
//         }else{
//             i += 1;
//         }
//     }
// }

// fn evenn_value(v:&Vec<i32>)-> Vec<i32>{
//     let mut new_vec = Vec::new();
//     for &value in v {
//         if value % 2 == 0 {
//             new_vec.push(value)
//         }
//     }
//     return new_vec;
// }


// fn main(){
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
    
//     let result = evenn_value(& vec);
//     odd_value(&mut vec);
//     println!("{:?}",result);
//     println!("{:?}",vec);
// }



// fn main(){
//     let numbers = vec![1,2,3];
//     for number in numbers {
//         println!("{}",number);
//     }
// }


// fn main(){
//     let numbers: Vec<i32> = vec![1,2,3];
//     for number in numbers {
//         println!("{}",number);
//     }
// }

// use std::io;
// fn main(){
//     println!("Enter the size of Array");
//     let mut input = String::new();
//         io::stdin()
//         .read_line(&mut input)
//         .unwrap();

//     // let input:<u32>  = input.trim().parse().unwrap();

//     let vec:Vec<i32>= input
//         .trim()
//         .split_whitespace()
//         .map(|x| x.parse().unwrap())
//         .collect();

//     println!("{:?}",vec)
        
// }