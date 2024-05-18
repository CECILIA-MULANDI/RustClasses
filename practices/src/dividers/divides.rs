//  program that takes in an input from terminal --2 and divides first/second 
// n/0
// integer overflow
use std::io;
pub fn divides(){

    println!("Enter the first number");
    let mut user_input1=String::new();
    io::stdin().read_line(&mut user_input1).expect("No line was read");
    let num1:u32=user_input1.trim().parse().expect("Not a number");
  

    println!("Enter the second number");
    let mut user_input2=String::new();
    io::stdin().read_line(&mut user_input2).expect("No line was read");
    let num2:u32=user_input2.trim().parse().expect("Not a number");
    
    let division = if num2 !=0{
        Some(num1/num2).unwrap()
    } 
    else{
        None.unwrap()
    };
    
    println!("{:?}",division);
}